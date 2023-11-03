function repeatNtimes {
    param (
        # its the first param!
        [Parameter(position = 1 , Mandatory = $true)]
        [string]
        $string,
        # its the 2nd param
        [Parameter(position = 2 , Mandatory = $true)]
        [int]
        $nTimes
    )
    echo ' '
    for ($i = 0; $i -lt $nTimes; $i++) {
        echo "`toutput: $string"
    }
    echo ' '
}

function usage {
    $errMsg = "usage: ./main.ps1 'some string' 4"
    echo $errMsg
    exit 0
}
function argParse {
    if ($args.count -lt 2) {
        usage
    }
    echo '[info]    passed count check'

    $isString = $args[0] -is [string]
    if (-Not $isString) {
        # argv is broken in this language
        usage       # bash like function calls
    }
    echo '[info]    passed isString check'
    $isInteger = $args[1] -is [int]
    if (-Not $isInteger) {
        usage
    }
    echo '[info]    passed isInteger check'
    echo '[info]    pass all checks!'
}
function main {
    argParse @args
    repeatNtimes @args
}
main @args