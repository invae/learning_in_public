Start-PodeServer -Thread 2 {
    # server entry point
    Add-PodeEndpoint -Address 127.0.0.1 -Port 8080 -Protocol Http
    Set-PodeViewEngine -Type Pode 
    Enable-PodeSessionMiddleware -Duration 120 -Extend

    $AuthScheme = 'Login'
    New-PodeAuthScheme -Form | Add-PodeAuth -Name $AuthScheme -FailureUrl '/login' -SuccessUrl '/secret' -ScriptBlock {
        param($username, $password)
    
        # here you'd check a real user storage, this is just for example
        if ($username -eq 'x' -and $password -eq 'x') {
            return @{
                User = @{
                    ID ='M0R7Y302'
                    Name = 'whoami'
                    Type = 'Human'
                }
            }
        }
    
        # aww geez! no user was found
        return @{ Message = 'Invalid details supplied' }
    }
    Add-PodeAuthSession -Name 'SessionAuth' -FailureUrl '/login'
    
    Add-PodeRoute -Method Get -Path '/login' -Authentication $AuthScheme -Login -ScriptBlock {
        Write-PodeViewResponse -Path 'auth-login' -FlashMessages
    }
    Add-PodeRoute -Method Post -Path '/login' -Authentication $AuthScheme -Login

    # how do i use a label here? 4 instances of logout to modify if need to change
    Add-PodeRoute -Method Post -Path '/logout' -Authentication SessionAuth -Logout -ScriptBlock {
        Write-PodeViewResponse -Path 'logout'
    }
    Add-PodeRoute -Method Get -Path '/logout' -ScriptBlock {
        Write-PodeViewResponse -Path 'logout'
    }

    Add-PodeRoute -Method Get -Path '/secret' -Authentication $AuthScheme -ScriptBlock {
        $WebEvent.Session.Data.Views++

        Write-PodeViewResponse -Path 'secret' -Data @{
            Username = $WebEvent.Auth.User.Name;
            Views = $WebEvent.Session.Data.Views;
        }
    }
    Add-PodeRoute -Method Post -Path '/secret' -Authentication $AuthScheme -ScriptBlock {
        $WebEvent.Session.Data.Views++

        Write-PodeViewResponse -Path 'secret' -Data @{
            Username = $WebEvent.Auth.User.Name;
            Views = $WebEvent.Session.Data.Views;
            clearText = $WebEvent.data.clearText;
            base64 = $WebEvent.data.base64;
        }
    }

    Add-PodeRoute -Method Get -Path '/example' -ScriptBlock {
        Write-PodeViewResponse -Path 'example'
    }
    Add-PodeRoute -Method Get -Path '/' -ScriptBlock {
        Write-PodeJsonResponse -Value @{ 'value' = 'yo waddup, world!' }
    }
}