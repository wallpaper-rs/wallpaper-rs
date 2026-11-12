use super::lamped::{LONG, ROPE, SPRITE, TRAIL};
use super::{B, LOOK, Look, Swarms, buffer};
use crate::Renderer;
use crate::scene::pipes::Mode;

impl Swarms {
    pub(crate) fn upload(&mut self, r: &Renderer, uniform: &wgpu::Buffer) {
        self.framed(r, uniform);
        let needed = self.looks.len() as u64 * LOOK;
        if needed == 0 {
            return;
        }
        if self.looked.as_ref().is_none_or(|(b, _)| b.size() < needed) {
            let buffer = buffer(r, needed.next_power_of_two().max(4 * LOOK), B::UNIFORM | B::COPY_DST);
            let size = wgpu::BufferSize::new(size_of::<Look>() as u64);
            let look =
                wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer: &buffer, offset: 0, size });
            let entries = [
                wgpu::BindGroupEntry { binding: 0, resource: look },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ];
            let desc = wgpu::BindGroupDescriptor { label: None, layout: &self.layouts[1], entries: &entries };
            let group = r.device.create_bind_group(&desc);
            self.looked = Some((buffer, group));
            self.created += 2;
        }
        self.bytes.clear();
        self.bytes.resize(needed as usize, 0);
        for (chunk, look) in self.bytes.chunks_mut(LOOK as usize).zip(&self.looks) {
            chunk[..size_of::<Look>()].copy_from_slice(bytemuck::bytes_of(look));
        }
        self.looked.iter().for_each(|(buffer, _)| r.queue.write_buffer(buffer, 0, &self.bytes));
    }

    pub(crate) fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        flock: usize,
        slot: u32,
        (mode, bend): (Mode, bool),
        lamps: &mut super::Lamps<'_>,
    ) {
        let flight = self.flights.get(flock).and_then(Option::as_ref);
        let pipes = (self.pipelines.get(&mode), self.ribbons.get(&mode));
        let (Some(flight), Some((_, group)), (Some(pipeline), Some(ribbon)), Some(herd)) =
            (flight, &self.looked, pipes, self.herds.get(flock))
        else {
            return;
        };
        let first = (u64::from(slot) / LOOK) as usize;
        for (p, store) in flight.stores.iter().enumerate() {
            let Some(&n) = self.kitted.get(first + p) else { continue };
            let Some(kit) = self.kits.get(n) else { continue };
            if bend != kit.bend.is_some() {
                continue;
            }
            let look = self.looks.get(first + p).map_or(0.0, |l| l.extra[1]);
            pass.set_bind_group(0, group, &[slot + (p as u64 * LOOK) as u32]);
            pass.set_bind_group(1, &kit.tex, &[]);
            let (alloc, k) = (u64::from(herd.bake.allocs[p]), u64::from(herd.bake.trails[p]));
            let lit = bend || self.looks.get(first + p).is_some_and(|l| l.light[0] > 0.5);
            let lamp = if lit { self.lamp(n, lamps) } else { None };
            if bend && lamp.is_none() {
                continue;
            }
            let how = (mode, bend, lamp.is_some());
            if let Some(group) = &lamp {
                pass.set_bind_group(2, group, &[]);
            }
            let trail = match (&store.trail, &store.gpu) {
                (Some(t), _) => Some((t, 16)),
                (None, Some((state, ..))) if k > 0 => Some((state, (4 + u64::from(herd.bake.marks[p])) * 4)),
                _ => None,
            };
            let cords = self.cords.get(&mode);
            if let Some((buffer, at)) = trail.filter(|_| look >= 2.0) {
                let size = alloc * k * 16 - 16;
                match cords.filter(|_| kit.long).and_then(|[_, long]| self.chosen(how, LONG, long)) {
                    Some(long) => {
                        pass.set_pipeline(long);
                        [at - 16, at, at + 16, at + 32].iter().enumerate().for_each(|(i, from)| {
                            pass.set_vertex_buffer(i as u32, buffer.slice(*from..*from + size))
                        });
                        let pieces = self.looks.get(first + p).map_or(0, |l| l.strand[0] as u32) + 2;
                        pass.draw(0..2 * pieces, 0..(alloc * k - 1) as u32);
                    }
                    None if !kit.long => {
                        if let Some(ribbon) = self.chosen(how, TRAIL, ribbon) {
                            pass.set_pipeline(ribbon);
                            pass.set_vertex_buffer(0, buffer.slice(at..at + size));
                            pass.set_vertex_buffer(1, buffer.slice(at + 16..at + 16 + size));
                            pass.draw(0..4, 0..(alloc * k - 1) as u32);
                        }
                    }
                    None => {}
                }
            }
            let roped = store.rope.as_ref().filter(|_| kit.rope.is_some());
            if let Some((rope, cord)) = roped.zip(cords.and_then(|[cord, _]| self.chosen(how, ROPE, cord))) {
                let (at, size) = (super::flight::ROPE_AT, (wprs_scene::ROPE_POINTS as u64 - 1) * 48);
                pass.set_pipeline(cord);
                pass.set_vertex_buffer(0, rope.slice(at..at + size));
                pass.set_vertex_buffer(1, rope.slice(at + 48..at + 48 + size));
                pass.draw(0..4, 0..(wprs_scene::ROPE_POINTS - 1) as u32);
            }
            let sorted = kit.order.and_then(|(o, _)| store.stacks.iter().find(|s| s.order == o));
            let variant = if bend { 2 } else { usize::from(lamp.is_some()) };
            let own = kit.stage.as_ref().zip(self.framed.as_ref()).and_then(|((key, ..), framed)| {
                let found = |v: usize| self.stages.get(&(mode, *key, v)).map(|s| (s, v));
                Some((found(variant).or_else(|| found(0).filter(|_| !bend))?, framed))
            });
            match (own, &lamp) {
                (Some(((stage, v), framed)), lamp) => {
                    pass.set_pipeline(stage);
                    pass.set_bind_group(2, framed, &[]);
                    if let Some(group) = lamp.as_ref().filter(|_| v > 0) {
                        pass.set_bind_group(3, group, &[]);
                    }
                }
                (None, _) => {
                    let seen = self.seen.get(first + p).and_then(Option::as_ref).filter(|_| !how.1 && !how.2);
                    match (seen.zip(self.seens.get(&mode)), self.chosen(how, SPRITE, pipeline)) {
                        (Some((group, seen)), _) => {
                            pass.set_pipeline(seen);
                            pass.set_bind_group(2, group, &[]);
                        }
                        (None, Some(chosen)) => pass.set_pipeline(chosen),
                        (None, None) => continue,
                    }
                }
            }
            pass.set_vertex_buffer(0, sorted.map_or(&store.particles, |s| &s.sorted).slice(..));
            pass.draw(0..if kit.look.hull[1][0] > 0.0 { 8 } else { 4 }, 0..herd.bake.allocs[p]);
        }
    }

    fn chosen<'s>(
        &'s self,
        (mode, bend, lit): (Mode, bool, bool),
        kind: usize,
        plain: &'s wgpu::RenderPipeline,
    ) -> Option<&'s wgpu::RenderPipeline> {
        match (bend, lit) {
            (true, _) => self.bends[kind].as_ref(),
            (false, true) => self.lits.get(&(mode, kind)).or(Some(plain)),
            (false, false) => Some(plain),
        }
    }
}
