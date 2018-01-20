use std::io::Error;
use mdns;

use accessory::AccessoryT;

use config::Config;
use db::storage::Storage;
use db::database::Database;
use db::file_storage::FileStorage;
use pin;
use protocol::device::Device;
use protocol::secured_device::SecuredDevice;

pub struct IpTransport<S: Storage, D: Storage> {
    config: Config,
    storage: S,
    database: Database<D>,
    //responder: mdns::Responder,
    secured_device: SecuredDevice,
}

impl/*<A: AccessoryT>*/ IpTransport<FileStorage, FileStorage> {
    fn new_single_device(mut config: Config/*, accessory: A*/) -> Result<IpTransport<FileStorage, FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;
        let pin = pin::new(&config.pin)?;

        config.load(&storage);

        // TODO - don't move config.id
        let secured_device = SecuredDevice::new(config.id, pin, &database)?;

        let ip_transport = IpTransport {
            config: config,
            storage: storage,
            database: database,
            secured_device: secured_device,
        };

        Ok(ip_transport)
    }
}