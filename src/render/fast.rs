use crate::render::RenderSettings;
pub struct RenderSettingsFast {}

impl RenderSettingsFast {
    pub fn new() -> Self {
        return RenderSettingsFast {};
    }
}
impl RenderSettings for RenderSettingsFast {}
