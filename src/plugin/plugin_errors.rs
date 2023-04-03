use error_chain::error_chain;

error_chain! {
    errors {
        // Plugins
        PluginIdExists
        PluginNotLoaded

        // Registry
        RegistryIdExists

        // Downloads
        DownloadFailedToConnect
        DownloadFailedToCreateDestFile
        DownloadErrorDuringDownload
        DownloadNoFileSize
    }
}