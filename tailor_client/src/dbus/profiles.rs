use zbus::{fdo, proxy};

#[proxy(
    interface = "com.tux.Tailor.Profiles",
    default_service = "com.tux.Tailor",
    default_path = "/com/tux/Tailor"
)]
pub trait Profiles {
    async fn add_profile(&self, name: &str, value: &str) -> fdo::Result<()>;

    async fn get_profile(&self, name: &str) -> fdo::Result<String>;

    async fn list_profiles(&self) -> fdo::Result<Vec<String>>;

    async fn remove_profile(&self, name: &str) -> fdo::Result<()>;

    async fn rename_profile(&self, from: &str, to: &str) -> fdo::Result<Vec<String>>;

    async fn set_active_profile_name(&self, name: &str) -> fdo::Result<()>;

    async fn get_active_profile_name(&self) -> fdo::Result<String>;

    async fn get_number_of_fans(&self) -> fdo::Result<u8>;

    async fn get_led_devices(&self) -> fdo::Result<String>;

    async fn reload(&self) -> fdo::Result<()>;
}
