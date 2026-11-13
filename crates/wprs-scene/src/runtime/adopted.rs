use crate::{Layer, MAX_DEPTH, Runtime, ScriptOutputs};
use std::collections::BTreeSet;

pub(super) fn targets<'a>(layers: impl Iterator<Item = &'a Layer>) -> BTreeSet<String> {
    let each = layers.flat_map(|l| {
        let masks = l.mask.iter().chain(l.effects.iter().filter_map(|e| e.mask.as_ref()));
        let fed = l.effects.iter().flat_map(|e| e.params.values()).filter_map(|v| v.binding()?.layer.clone());
        let lit = match &l.kind {
            crate::Kind::Light(light) => light.texture.as_ref().and_then(|t| t.binding()?.layer.clone()),
            _ => None,
        };
        masks.filter_map(|m| m.layer.clone()).chain(fed).chain(lit)
    });
    each.collect()
}

impl Runtime {
    pub(crate) fn parents(&self, outputs: &mut ScriptOutputs) {
        let wanted: Vec<(String, String)> = (outputs.layers.iter())
            .filter_map(|(id, set)| {
                set.parent.as_ref().filter(|p| !p.is_empty()).map(|p| (id.clone(), p.clone()))
            })
            .collect();
        for (id, parent) in wanted {
            let parent_of = |at: &str| match outputs.layers.get(at).and_then(|s| s.parent.as_deref()) {
                Some("") => None,
                Some(p) => Some(p.to_owned()),
                None => self.scene.layer(at).and_then(|l| l.parent.clone()),
            };
            let (mut at, mut depth, mut fine) = (Some(parent.clone()), 0, true);
            while let Some(step) = at {
                fine &= step != id && depth < MAX_DEPTH && self.scene.layer(&step).is_some_and(Layer::drawn);
                (at, depth) = (if fine { parent_of(&step) } else { None }, depth + 1);
            }
            if !fine {
                outputs.layers.get_mut(&id).iter_mut().for_each(|set| set.parent = None);
            }
        }
    }

    pub(crate) fn molds(&self, outputs: &mut ScriptOutputs) {
        outputs.meshes.retain(|id, mesh| self.scene.pools.get(id).is_some_and(|pool| mesh.fits(pool)));
    }

    pub(crate) fn rematte(&mut self) {
        let mut all: Vec<&Layer> = self.scene.layers().collect();
        let mut stack: Vec<&Layer> = self.spawned().collect();
        while let Some(l) = stack.pop() {
            all.push(l);
            stack.extend(l.children());
        }
        let wanted = targets(all.into_iter());
        let mut mattes = BTreeSet::new();
        super::draw::holds(&self.scene.layers, &wanted, &mut mattes);
        self.spawned().for_each(|l| _ = super::draw::holds(std::slice::from_ref(l), &wanted, &mut mattes));
        self.mattes = mattes;
    }

    pub(crate) fn mirror_sheets(&self, outputs: &mut ScriptOutputs) {
        if outputs.rigs.is_empty() || self.scene.sheets.is_empty() {
            return;
        }
        let mut pairs: Vec<(String, String)> =
            self.scene.sheets.iter().map(|(f, s)| (f.clone(), s.clone())).collect();
        for (template, layer) in &self.spawned {
            let (Some(source), feeders) = (self.scene.templates.get(template), self.scene.rooted.get(template))
            else {
                continue;
            };
            let to = crate::copies::renames(source, &layer.id, feeders.map_or(&[][..], |f| &f[..]));
            for (f, s) in &self.scene.sheets {
                if let (Some(f), Some(s)) = (to.get(f), to.get(s)) {
                    pairs.push((f.clone(), s.clone()));
                }
            }
        }
        for (feeder, source) in pairs {
            if let Some(set) = outputs.rigs.get(&source).cloned() {
                outputs.rigs.insert(feeder, set);
            }
        }
    }

    pub fn script(&self) -> &ScriptOutputs {
        &self.script
    }

    pub fn mesh(&self, pool: &str) -> Option<&crate::Mesh> {
        self.script.meshes.get(pool).map(|m| &**m)
    }
}
