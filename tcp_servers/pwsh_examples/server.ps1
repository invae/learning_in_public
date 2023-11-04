# https://learn.microsoft.com/en-us/dotnet/api/system.net.sockets.tcplistener?view=net-7.0


$interface = "127.0.0.1"
$port = "9001"

$tcpConnection = New-Object System.Net.Sockets.TcpListener($interface, $port)
echo "starting to listen for connections on $interface`:$port"
$tcpConnection.start()  # BLOCKING
$client = $tcpConnection.AcceptTcpClient()
if ($client.Connected) {
    echo "client connected!"
}


$tcpConnection.stop()