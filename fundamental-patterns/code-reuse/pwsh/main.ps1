$ProjectRoot = (pwd).path

<#
https://stackoverflow.com/questions/37758514/how-to-declare-an-array-of-strings-on-multiple-lines
this section we define our modules so they can be imported
#>
$utils = @()
$utils += "fn_printer"
$utils += "fn_hello"


function include-utils {
    foreach ($item in $utils) {
        Import-Module -Name  $ProjectRoot/utils/$item.psm1 # -verbose
    }
}
function remove-utils {
    # remove modules to protect shell env
    foreach ($item in $utils) {
        Remove-Module -Name $item #-verbose
    }
}


function main {
    include-utils
    
    echo "main: $utils"
        
    <# 
    naming things will be very important
    func name does not have a slug to indicate origin
    #>
    fn_printer
    fn_hello
    
    remove-utils
}
main @args 


