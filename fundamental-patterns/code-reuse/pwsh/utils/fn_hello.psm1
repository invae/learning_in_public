function fn_hello {
    $something = 'hello function'
    echo $something
}
Export-ModuleMember -Function fn_hello