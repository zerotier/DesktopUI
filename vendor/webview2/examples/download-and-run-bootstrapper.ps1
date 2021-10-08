Add-Type -AssemblyName PresentationFramework

function install() {
    if ([System.Windows.MessageBox]::Show("The WebView2 Runtime must be installed to use this program. Install now?", 'Install WebView2 Runtime?', 'YesNo', 'Question') -eq "Yes") {
        $fileName = [System.IO.Path]::GetTempFileName()

        try {
            try {
                $newName = [System.IO.Path]::ChangeExtension($fileName, ".exe")
                Move-Item $fileName $newName
                $fileName = $newName

                Write-Host "Downloading the WebView2 Runtime Bootstrapper to" $fileName
                Invoke-WebRequest -Uri 'https://go.microsoft.com/fwlink/p/?LinkId=2124703' -OutFile $fileName
                Write-Host "Installing..."
                Start-Process -FilePath $fileName -Verb runas -Wait
            } finally {
                Write-Host Deleting $fileName
                Remove-Item $fileName
            }
        } catch {
            Write-Host $_
            [System.Windows.MessageBox]::Show($_, 'Error', 'Ok', 'Error') | Out-Null
            exit 1
        }
    } else {
        exit 1
    }
}

install
