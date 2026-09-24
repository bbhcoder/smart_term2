pub fn generate() -> String {
    String::from(
        "_smart_term_preexec() {\n\
            local cmd=\"$BASH_COMMAND\"\n\
            smart hook pre-exec \"$cmd\"\n\
        }\n\
        _smart_term_precmd() {\n\
            local exit_code=$?\n\
            smart hook pre-cmd $exit_code\n\
        }\n\
        trap '_smart_term_preexec' DEBUG\n\
        PROMPT_COMMAND=\"_smart_term_precmd; ${PROMPT_COMMAND:-}\"\n"
    )
}
