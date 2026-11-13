use crate::picture::{write, write_png};
use crate::relief::qobject::Relief;
use crate::reliefjobs::{forget, url};
use crate::reliefwork::{Target, frame, outline, overlay, regions, survey};
use cxx_qt_lib::QString;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use wprs_depth::Places;

pub(crate) const OVERLAY: usize = 0;
pub(crate) const OUTLINE: usize = 1;
pub(crate) const SURVEY: usize = 2;
pub(crate) const CHART: usize = 3;
pub(crate) const PEEK: usize = 4;
pub(crate) const KINDS: usize = 5;

pub(crate) enum Made {
    File(PathBuf),
    Text(String),
    Chart(PathBuf, String),
    Nothing,
}

pub(crate) fn work(kind: usize, file: &Path, args: &str) -> anyhow::Result<Made> {
    let (head, edit) = args.split_once('\n').unwrap_or(("", args));
    let (places, target) = (Places::default(), Target::new(file, if kind == SURVEY { args } else { edit })?);
    Ok(match kind {
        OVERLAY => Made::File(write(&overlay(&places, &target, head == "true")?)?),
        OUTLINE => {
            let cuts: Vec<f32> = serde_json::from_str(head)?;
            Made::File(write_png(&outline(&places, &target, &cuts)?)?)
        }
        CHART => {
            let (image, text) = regions(&places, &target)?;
            Made::Chart(write_png(&image)?, text)
        }
        PEEK => {
            let pointer: Vec<f32> = serde_json::from_str(head)?;
            let at = [pointer.first().copied().unwrap_or(0.0), pointer.get(1).copied().unwrap_or(0.0)];
            frame(&places, &target, at)?.map_or(Ok(Made::Nothing), |i| write_png(&i).map(Made::File))?
        }
        _ => Made::Text(survey(&places, &target)?),
    })
}

impl Relief {
    pub(crate) fn land(mut self: Pin<&mut Self>, kind: usize, made: anyhow::Result<Made>) {
        let made = match made {
            Err(e) => return self.as_mut().set_error(QString::from(&format!("{e:#}"))),
            Ok(made) => made,
        };
        match (kind, made) {
            (_, Made::Text(text)) => self.as_mut().set_survey(QString::from(&text)),
            (_, Made::Chart(path, text)) => {
                let old = self.chart.clone();
                self.as_mut().set_regions(QString::from(&text));
                self.as_mut().set_chart(url(&path));
                forget(&old);
            }
            (_, Made::File(path)) => {
                let old = match kind {
                    OVERLAY => self.overlay.clone(),
                    PEEK => self.frame.clone(),
                    _ => self.outline.clone(),
                };
                match kind {
                    OVERLAY => self.as_mut().set_overlay(url(&path)),
                    PEEK => self.as_mut().set_frame(url(&path)),
                    _ => self.as_mut().set_outline(url(&path)),
                }
                forget(&old);
            }
            (_, Made::Nothing) => {
                let old = self.frame.clone();
                self.as_mut().set_frame(QString::default());
                forget(&old);
            }
        }
    }
}
