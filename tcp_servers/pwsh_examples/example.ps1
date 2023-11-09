# https://stackoverflow.com/questions/44518636/powershell-tcp-server
# someone else's example
$Global:Listener = [HashTable]::Synchronized(@{})
$Global:CnQueue = [System.Collections.Queue]::Synchronized((New-Object System.collections.queue))
$Global:space = [RunSpaceFactory]::CreateRunspace()
$space.Open()
$space.SessionStateProxy.setVariable("CnQueue", $CnQueue)
$space.SessionStateProxy.setVariable("Listener", $Listener)
$Global:newPowerShell = [PowerShell]::Create()
$newPowerShell.Runspace = $space
$Timer = New-Object Timers.Timer
$Timer.Enabled = $true
$Timer.Interval = 1000

Register-ObjectEvent -SourceIdentifier MonitorClientConnection -InputObject $Timer -EventName Elapsed -Action {
    While($CnQueue.count -ne 0) {
        $client = $CnQueue.Dequeue()
        $newRunspace = [RunSpaceFactory]::CreateRunspace()
        $newRunspace.Open()
        $newRunspace.SessionStateProxy.setVariable("client", $client)
        $newPowerShell = [PowerShell]::Create()
        $newPowerShell.Runspace = $newRunspace
        $process = {
            $stream = $client.GetStream();
            # $reader = New-Object System.IO.StreamReader $stream
            [console]::WriteLine("Inside Processing")
            # You have client here so do whatever you want to do here.
            # This is a separate thread so if you write blocking code here, it will not impact any other part of the program
            $INFO_MSG = "[info]"
            $ERR_MSG  = "[errr]"
            $WARN_MSG = "[warn]"
            function strip_line($inString) {
                [string] $result
                for ($i = 0; $i -lt $inString.Length; $i++) {
                    if ($inString[$i] -eq 0x0a) {
                        # Write-Output "$WARN_MSG found a new line"
                        continue
                    }
                    if ($inString[$i] -eq 0) {
                        # Write-Output "$WARN_MSG found a null"
                        return $result
                    }
                    $result += $inString[$i]
                }
            }
            
            $buf_msg      = "WELCOME`n"
            $buf_bytes    = [Text.Encoding]::ASCII.GetBytes($buf_msg)
            $stream.Write($buf_bytes, 0, $buf_bytes.Length)
            $stream.Flush()
            Write-Output "$INFO_MSG serving client"
            $read_buffer  = [System.Byte[]]::CreateInstance([System.Byte],1024)
            
            $ack_msg      = "got your message!`n"
            $ack_bytes    = [Text.Encoding]::ASCII.GetBytes($ack_msg)
            
            while(($i = $stream.Read($read_buffer, 0, $read_buffer.Length)) -ne 0) {
                $msg = [Text.Encoding]::ASCII.GetString($read_buffer);
                $stripped_line = strip_line($msg).Trim()
                
                Write-Output "$INFO_MSG$stripped_line" #| Out-File /tmp/log
                
                # https://adamtheautomator.com/powershell-switch/
                switch -exact ("$stripped_line".Trim()) {
                    "test" { 
                        Write-Output "$WARN_MSG test switch is TODO"
                        # continue
                    }
                    "options" { 
                        Write-Output "$WARN_MSG options switch is TODO"
                        # continue
                    }
                    "hello" { 
                        Write-Output "$WARN_MSG hello switch is TODO"
                        # continue
                    }
                    "exit" { 
                        Write-Output "$WARN_MSG begin exit routine"
                        $buf_msg      = "bye for now!`n"
                        $buf_bytes    = [Text.Encoding]::ASCII.GetBytes($buf_msg)
                        $stream.Write($buf_bytes, 0, $buf_bytes.Length)
                        $stream.Flush()
                        $stream.Close()
                        $tcpConnection.stop()
                        Write-Output "$WARN_MSG function returns"
                        return
                        # continue
                    }
                    Default {
                        Write-Output "$WARN_MSG sending ACK to client"
                        $stream.Write($ack_bytes, 0, $ack_bytes.Length)
                        $stream.Flush()
                    }
                }

                $read_buffer = [System.Byte[]]::CreateInstance([System.Byte],1024)
            }
        }
        $jobHandle = $newPowerShell.AddScript($process).BeginInvoke()
        #jobHandle you need to save for future to cleanup
    }
}
$listener = {
    $Listener['listener'] = New-Object System.Net.Sockets.TcpListener("127.0.0.1", "1234")
    $Listener['listener'].Start()
    [console]::WriteLine("Listening on :1234")
    while ($true) {
        $c = $Listener['listener'].AcceptTcpClient()
        If($c -ne $Null) {
            [console]::WriteLine("{0} >> Accepted Client " -f (Get - Date).ToString())
            $CnQueue.Enqueue($c)
        }
        Else {
            [console]::WriteLine("Shutting down")
            Break
        }
    }
}
$Timer.Start()
$Global:handle = $newPowerShell.AddScript($listener).BeginInvoke()
