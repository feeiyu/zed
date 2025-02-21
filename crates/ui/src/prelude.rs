//! The prelude of this crate. When building UI in Zed you almost always want to import this.

pub use gpui::prelude::*;
pub use gpui::{
    div, px, relative, rems, AbsoluteLength, AnyElement, App, Context, DefiniteLength, Div,
    Element, ElementId, InteractiveElement, ParentElement, Pixels, Rems, RenderOnce, SharedString,
    Styled, Window,
};

pub use component::{example_group, example_group_with_title, single_example, ComponentPreview};
pub use ui_macros::IntoComponent;

pub use crate::styles::{rems_from_px, vh, vw, PlatformStyle, StyledTypography, TextSize};
pub use crate::traits::clickable::*;
pub use crate::traits::disableable::*;
pub use crate::traits::fixed::*;
pub use crate::traits::styled_ext::*;
pub use crate::traits::toggleable::*;
pub use crate::traits::visible_on_hover::*;
pub use crate::DynamicSpacing;
pub use crate::{h_container, h_flex, v_container, v_flex};
pub use crate::{
    h_group, h_group_lg, h_group_sm, h_group_xl, v_group, v_group_lg, v_group_sm, v_group_xl,
};
pub use crate::{Button, ButtonSize, ButtonStyle, IconButton, SelectableButton};
pub use crate::{ButtonCommon, Color};
pub use crate::{Headline, HeadlineSize};
pub use crate::{Icon, IconName, IconPosition, IconSize};
pub use crate::{Label, LabelCommon, LabelSize, LineHeightStyle};
pub use theme::ActiveTheme;
