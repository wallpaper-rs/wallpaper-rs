use crate::pack::Pack;
use crate::props::{GRANTS, Grants, grants};
use crate::{Error, said, store};
use std::fs;
use std::sync::Arc;
use wprs_core::model::Input;
use wprs_scene::{Runtime, Uses, Values};

const DECLINED: &str = "declined.json";

const NAMES: [(&str, Input); 7] = [
    ("audio", Input::Audio),
    ("audio_network", Input::AudioNetwork),
    ("touch", Input::Pointer),
    ("taps", Input::Clicks),
    ("media", Input::Media),
    ("media_network", Input::MediaNetwork),
    ("network", Input::Network),
];

#[derive(Clone, Copy)]
pub(crate) struct Wants {
    pub(crate) audio: bool,
    pub(crate) touch: bool,
    pub(crate) taps: bool,
    pub(crate) media: bool,
    pub(crate) network: bool,
    pub(crate) web: bool,
}

impl Wants {
    fn wants(self, input: Input) -> bool {
        match input {
            Input::Audio => self.audio,
            Input::AudioNetwork => self.audio && self.network,
            Input::Pointer => self.touch,
            Input::Clicks => self.taps,
            Input::Media => self.media,
            Input::MediaNetwork => self.media && self.network,
            Input::Network => self.web && self.network,
            _ => false,
        }
    }
}

pub(crate) fn wants(pack: &Pack, values: Values) -> (Uses, Wants) {
    let mut runtime = Runtime::new(Arc::clone(&pack.scene), values);
    pack.inputs.into_iter().for_each(|allowed| runtime.declare(allowed));
    let ran = runtime.uses();
    let [audio, touch, taps] = pack.web.as_ref().map_or([ran.audio, ran.cursor, ran.clicks], |w| w.uses);
    let media = pack.web.as_ref().map_or(ran.media, |w| w.media);
    let network = pack.web.as_ref().is_some_and(|w| !w.network.is_empty());
    (ran, Wants { audio, touch, taps, media, network, web: pack.web.is_some() })
}

fn declined() -> Grants {
    let read = store::dir().ok().and_then(|dir| fs::read(dir.join(DECLINED)).ok());
    read.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
}

fn input(pack: &Pack, name: &str) -> Result<Input, Error> {
    let network = pack.web.as_ref().is_some_and(|w| !w.network.is_empty());
    let found = NAMES.iter().find(|(n, _)| *n == name).map(|&(_, input)| input);
    let networked = |input: &Input| {
        network || !matches!(input, Input::Network | Input::AudioNetwork | Input::MediaNetwork)
    };
    found
        .filter(networked)
        .ok_or_else(|| said("android.audio_touch_taps", "%1 is not audio, touch, taps or media", &[&name]))
}

pub(crate) fn undecided(pack: &Pack, wants: Wants) -> Vec<&'static str> {
    let (grants, declined) = (grants(), declined());
    let (given, refused) = (grants.get(&pack.id), declined.get(&pack.id));
    let answered = |input: &Input| {
        given.is_some_and(|g| g.contains(input)) || refused.is_some_and(|d| d.contains(input))
    };
    let under = |input: Input| match input {
        Input::AudioNetwork => Some(Input::Audio),
        Input::MediaNetwork => Some(Input::Media),
        _ => None,
    };
    let open = |input: &Input| under(*input).is_none_or(|first| !refused.is_some_and(|d| d.contains(&first)));
    NAMES
        .iter()
        .filter(|(_, input)| wants.wants(*input) && !answered(input) && open(input))
        .map(|&(name, _)| name)
        .collect()
}

pub(crate) fn decide(pack: &Pack, allow: &[String], deny: &[String]) -> Result<(), Error> {
    let read = |names: &[String]| names.iter().map(|n| input(pack, n)).collect::<Result<Vec<_>, _>>();
    let (allow, deny) = (read(allow)?, read(deny)?);
    if let Some(both) = allow.iter().find(|input| deny.contains(input)) {
        let name = NAMES.iter().find(|(_, i)| i == both).map_or("", |(n, _)| n);
        return Err(said("android.decided_twice", "%1 is both allowed and declined", &[&name]));
    }
    let (mut grants, mut declined) = (grants(), declined());
    let (given, refused) =
        (grants.entry(pack.id.clone()).or_default(), declined.entry(pack.id.clone()).or_default());
    for input in allow {
        given.insert(input);
        refused.remove(&input);
    }
    for input in deny {
        refused.insert(input);
        given.remove(&input);
    }
    grants.retain(|_, inputs| !inputs.is_empty());
    declined.retain(|_, inputs| !inputs.is_empty());
    let dir = store::dir()?;
    store::write(&dir.join(GRANTS), &serde_json::to_vec_pretty(&grants)?)?;
    store::write(&dir.join(DECLINED), &serde_json::to_vec_pretty(&declined)?)?;
    store::changed();
    Ok(())
}

pub(crate) fn grant(pack: &Pack, name: &str, on: bool) -> Result<(), Error> {
    let one = [name.to_owned()];
    if on { decide(pack, &one, &[]) } else { decide(pack, &[], &one) }
}
