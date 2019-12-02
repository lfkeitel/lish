;; Macro to define a function
(define-syntax defun (name args body)
    `(setf '%name (lambda %args %body)))

;; Macro to define an alias
;; This macro creates another macro that allows any number of arguments to
;; be given allowing aliases to take more flags or commands.
(define-syntax alias (name body)
    `(define-syntax %name (&rest) `(%@body !%@rest)))

;; ANSI color codes
(include "colors.lisp")

;; My colorful prompt
(defun prompt ()
    (string-concat
        color_green
        (get-key (capc whoami) :stdout) ; capc = capture call
        " "
        color_light_blue
        (pwd)
        color_reset
        " ➤ "))

;; Cargo aliases
(alias cb (cargo build))
(alias cr (cargo run))

;; git aliases
(alias g (git))
(alias gc (git commit))
(alias gca (git commit -v -a))
(alias gs (git status))
(alias gp (git push))
(alias gl (git pull))
(alias ga (git add))
(alias gaa (git add -a))
(alias gd (git diff))
(alias gds (git diff --staged))

(define config-file (current-file))

(defun reload () (include config-file))

;; Notice that this file loaded correctly
(define lishrc :t)
(print "lishrc loaded")
