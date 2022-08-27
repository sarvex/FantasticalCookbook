use bevy::prelude::Res;
use crate::MixerRecipeIden;
use crate::registry::{Registry};
use bevy::prelude::*;
use crate::element::Element;
use crate::ui::CraftType::MIXER;

#[derive(PartialEq, Default, Debug, Clone)]
pub struct MixerRecipe {
    pub first: Element,
    pub second: Element,
    pub result: Element,
}

impl MixerRecipe {
    pub const UTTER_ICE_CREAM: MixerRecipe = MixerRecipe::new(Element::SHAVED_ICE, Element::LEGEND_DAIRY, Element::UTTER_ICE_CREAM);
    pub const BREAD_DOUGH: MixerRecipe = MixerRecipe::new(Element::FANTASY_FLOUR, Element::YETI_WATER, Element::BREAD_DOUGH);
    pub const ICE_CREAM_SANDWICH: MixerRecipe = MixerRecipe::new(Element::ELVEN_BREAD, Element::UTTER_ICE_CREAM, Element::ICE_CREAM_SANDWICH);
    //pub const MAYO: MixerRecipe = MixerRecipe::new(Element::GRIFFON_EGG, Element::GRIFFON_EGG, Element::);
    //pub const RANCH: MixerRecipe = MixerRecipe::new(Element::GRIFFON_EGG, Element::GRIFFON_EGG, Element::);
    pub const SALAD_TOPPING: MixerRecipe = MixerRecipe::new(Element::DICED_CROUTONS, Element::RANCH, Element::SALAD_TOPPING);
    pub const SALAD: MixerRecipe = MixerRecipe::new(Element::SIREN_SEAWEED, Element::RANCH, Element::SALAD);
    //pub const SPICY_SPREAD: MixerRecipe = MixerRecipe::new(Element::SPICY_SPREAD, Element::, Element::UTTER_ICE_CREAM);
    pub const SPICY_TOAST: MixerRecipe = MixerRecipe::new(Element::SPICY_SPREAD, Element::ELVEN_TOAST, Element::SPICY_TOAST);
    pub const COOKED_PORK: MixerRecipe = MixerRecipe::new(Element::RAW_PORK, Element::BOILING_WATER, Element::COOKED_PORK);
    pub const HARD_BOILED_EGG: MixerRecipe = MixerRecipe::new(Element::GRIFFON_EGG, Element::BOILING_WATER, Element::HARD_BOILED_EGG);

    pub const RECIPES: [MixerRecipe; 8] = [
        MixerRecipe::UTTER_ICE_CREAM,
        MixerRecipe::BREAD_DOUGH,
        MixerRecipe::ICE_CREAM_SANDWICH,
        //
        //
        MixerRecipe::SALAD_TOPPING,
        MixerRecipe::SALAD,
        //
        MixerRecipe::SPICY_TOAST,
        MixerRecipe::COOKED_PORK,
        MixerRecipe::HARD_BOILED_EGG
    ];

    pub const fn new(first: Element, second: Element, result: Element) -> Self {
        Self {
            first,
            second,
            result,
        }
    }

    pub fn id(&self) -> String {
        let id = format!("{}_{}_{}", self.first.id, self.second.id, self.result.id);
        return id;
    }
}

pub fn get_result(element_a: Element, element_b: Element, registry: &Res<Registry>) -> Option<Element> {
    let iden = MixerRecipeIden::new(element_a, element_b);
    if let Some(mr) = registry.mixer_recipe_registry.get(&iden) {
        Some(mr.result.clone())
    } else {
        None
    }
}