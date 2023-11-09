
function main {
    param (
        [Parameter(Position = 0, Mandatory = $True)]
        [String]
        $string
    )  
    echo $string
}
main $args[0]
