# https://learn.microsoft.com/en-us/dotnet/api/system.net.sockets.tcpclient?view=net-7.0
$srv_addr = "127.0.0.1"
$port = "9001"

$tcpConnection = New-Object System.Net.Sockets.TcpClient($srv_addr, $port)
$tcpStream = $tcpConnection.GetStream()

if ($tcpConnection.Connected) {
    echo "connected to server!"
}
