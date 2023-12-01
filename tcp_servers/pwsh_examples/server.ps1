# https://learn.microsoft.com/en-us/dotnet/api/system.net.sockets.tcplistener?view=net-7.0

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

function serve_client($client) {
    Write-Output "$INFO_MSG serving client"
    $stream         = $client.GetStream()
    $read_buffer    = [System.Byte[]]::CreateInstance([System.Byte],1024)
    
    $ack_msg      = "got your message!`n"
    $ack_bytes    = [Text.Encoding]::ASCII.GetBytes($ack_msg)
    
    while(($i = $stream.Read($read_buffer, 0, $read_buffer.Length)) -ne 0) {
        $msg = [Text.Encoding]::ASCII.GetString($read_buffer);
        $stripped_line = strip_line($msg).Trim()
        
        Write-Output "$INFO_MSG$stripped_line"
        
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


$some_block = {
    Write-Output "$INFO_MSG serving client"
    $stream         = $client.GetStream()
    $read_buffer    = [System.Byte[]]::CreateInstance([System.Byte],1024)
    
    $ack_msg      = "got your message!`n"
    $ack_bytes    = [Text.Encoding]::ASCII.GetBytes($ack_msg)
    
    while(($i = $stream.Read($read_buffer, 0, $read_buffer.Length)) -ne 0) {
        $msg = [Text.Encoding]::ASCII.GetString($read_buffer);
        $stripped_line = strip_line($msg).Trim()
        
        Write-Output "$INFO_MSG$stripped_line"
        
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
# CONSTANTS
$INFO_MSG = "[info]"
$ERR_MSG  = "[errr]"
$WARN_MSG = "[warn]"


$interface = "127.0.0.1"
$port = "9001"


while (1) {
    Write-Output "$INFO_MSG starting to listen for connections on $interface`:$port"
    $tcpConnection = New-Object System.Net.Sockets.TcpListener($interface, $port)

    try {
        $tcpConnection.start() # BLOCKING    
        $client = $tcpConnection.AcceptTcpClient()
    }
    catch {
        Write-Output "$ERR_MSG failed to bind to $interface`:$port. Addr in use?"
    }
   
    # Threading issues?
    # https://pode.readthedocs.io/en/latest/Servers/TCP/#wildcard
    if ($client.Connected) {
        Write-Output "$INFO_MSG client connected!"
        serve_client($client)
        # Invoke-Command $some_block -ArgumentList $client
    }

    # $prompt = Read-Host 'continue? [y/n]'
    # if ($prompt.ToLower() -eq 'n') {
    #     Exit
    # }
}
