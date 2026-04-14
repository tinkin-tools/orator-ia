use crate::domain::settings::entities::Settings;
use crate::domain::settings::repositories::SettingsRepository;
use crate::domain::shared::DomainResult;

pub struct GetSettingsUseCase<R: SettingsRepository> {
    repository: R,
}

impl<R: SettingsRepository> GetSettingsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> DomainResult<Settings> {
        self.repository.load_settings()
    }
}
