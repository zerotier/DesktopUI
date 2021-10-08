#[cfg(feature="windows-applicationmodel")] pub mod applicationmodel; // Windows.ApplicationModel
#[cfg(feature="windows-data")] pub mod data; // Windows.Data
#[cfg(feature="windows-devices")] pub mod devices; // Windows.Devices
pub mod foundation; // Windows.Foundation
#[cfg(feature="windows-gaming")] pub mod gaming; // Windows.Gaming
#[cfg(feature="windows-globalization")] pub mod globalization; // Windows.Globalization
#[cfg(feature="windows-graphics")] pub mod graphics; // Windows.Graphics
#[cfg(feature="windows-management")] pub mod management; // Windows.Management
#[cfg(feature="windows-media")] pub mod media; // Windows.Media
#[cfg(feature="windows-networking")] pub mod networking; // Windows.Networking
#[cfg(feature="windows-perception")] pub mod perception; // Windows.Perception
#[cfg(feature="windows-security")] pub mod security; // Windows.Security
#[cfg(feature="windows-services")] pub mod services; // Windows.Services
#[cfg(feature="windows-storage")] pub mod storage; // Windows.Storage
#[cfg(feature="windows-system")] pub mod system; // Windows.System
#[cfg(feature="windows-ui")] pub mod ui; // Windows.UI
#[cfg(feature="windows-web")] pub mod web; // Windows.Web
