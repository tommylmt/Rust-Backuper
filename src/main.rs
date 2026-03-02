fn main() {
    /* TODO:
        - Preflight check: 
            do we have a configuration file in /etc/rust-backuper/backuper.conf
            if not, create one and ask user to configure it
        - Savers:
            do a backup for each category of the backuper.conf files
        - Bundlers:
            pack elements with "archive = true" in config file in "zip"
        - Transporters:
            sync elements to the configured endpoint (Google Drive)
        - Cleaners:
            remove local backups once sync is OK
    */      
    println!("Hello, World!");
}
