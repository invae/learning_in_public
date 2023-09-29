VALID_ACTIONS = {
    "hello" => "hello",
    "test"  => "test",
}
VALID_ACTIONS.each do |k, v|
    action_source_code = './dispatch_functions/' + VALID_ACTIONS[k]
    require action_source_code
end

module Dispatcher
    def Dispatcher.entry(action)
        if VALID_ACTIONS[action] != nil
            defined_action = VALID_ACTIONS[action] + "_entry"
            method( defined_action ).call
        else
            $stdout.puts "VERB " + action + " is not implemented"
            $stdout.flush
        end
    end
end