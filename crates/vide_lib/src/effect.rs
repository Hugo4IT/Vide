use std::{any::Any, sync::MutexGuard};

use crate::render::{Renderer, RenderFunction};

#[macro_export] macro_rules! register_effect {
    ($name:ident, $dataname:ident) => {
        $crate::paste::paste! {
            #[allow(non_upper_case_globals)]
            static mut [<$name _ID>]: usize = usize::MAX;
            impl $crate::effect::RegisteredEffectData for $dataname {
                unsafe fn is_registered() -> bool {
                    [<$name _ID>] != usize::MAX
                }

                unsafe fn get_id() -> usize {
                    if [<$name _ID>] == usize::MAX {
                        [<$name _ID>] = $crate::effect::effect_counter();
                    }
                    [<$name  _ID>]
                }

                fn _new(renderer: &mut $crate::render::Renderer) -> Box<dyn std::any::Any> {
                    Box::new(<$name as $crate::effect::Effect>::new(renderer))
                }

                fn _render<'a>(backend: &'a Box<dyn std::any::Any>, params: &Box<dyn std::any::Any>, pass: std::sync::MutexGuard<wgpu::RenderPass<'a>>, frame: u64) {
                    $name::render(backend.as_ref().downcast_ref().unwrap(), params.as_ref().downcast_ref().unwrap(), pass, frame)
                }
            }
        }
    };
}

static mut COUNTER: usize = 0;
pub unsafe fn effect_counter() -> usize {
    COUNTER += 1;
    COUNTER - 1
}

pub enum EffectParameter {
    F64(f64),
}

pub trait Effect {
    fn new(renderer: &mut Renderer) -> Self;
}

pub trait RegisteredEffectData {
    unsafe fn is_registered() -> bool;
    unsafe fn get_id() -> usize;
    fn _new(renderer: &mut Renderer) -> Box<dyn Any>;
    fn _render<'a>(backend: &'a Box<dyn Any>, params: &Box<dyn Any>, pass: MutexGuard<'_, wgpu::RenderPass<'a>>, frame: u64);
}

pub struct EffectData {
    pub(crate) id: usize,
    pub(crate) params: Box<dyn Any>,
}

#[derive(Clone, Copy)]
pub struct EffectRegistrationPacket {
    pub id: usize,
    pub render_function: RenderFunction,
    pub init_function: fn(&mut Renderer)->Box<dyn Any>,
}