pub fn generate() -> String {
    String::from(
        "autoload -Uz add-zsh-hook\n\
        _smart_term_preexec() {\n\
            local cmd=\"$1\"\n\
            smart hook pre-exec \"$cmd\"\n\
        }\n\
        _smart_term_precmd() {\n\
            local exit_code=$?\n\
            smart hook pre-cmd $exit_code\n\
        }\n\
        add-zsh-hook preexec _smart_term_preexec\n\
        add-zsh-hook precmd _smart_term_precmd\n"
    )
}
