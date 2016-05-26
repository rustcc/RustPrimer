# Emacs

This section describes the Emacs (Version 24) of Rust configuration, assuming you have installed Emacs, and have experience in using Emacs. Specific installation and usage instructions, see the documentation online, not in this repeat.

In addition, the examples in this section is on the Mac OS, Linux is basically the same in the above.

Emacs for Windows users only for reference.

## 1.1 Introduction

Emacs is rust-mode provides syntax highlighting and elisp function, you can define a function to move the cursor around the Rust. There are several plug-ins provide additional features such as auto-completion and syntax checking dynamic.

! [] (../ Image / 03-editor-emacs-base.png)

## 1.2 Installing the plug

First, you need to add the code base to melpa your plugin list, you can install the plug-Rust need. The following code snippet on your `` `~ / .emacs.d / init.el``` file.

`` `
;; Add melpa repository to archives
(Add-to-list 'package-archives
    '( "Melpa". "Http://melpa.milkbox.net/packages/") t)

;; Initialize packages
(Package-initialize)

`` `

Run the following command to update the plug-in list.

- M-x eval-buffer
- M-x package-refresh-contents

Then, you can install plug-ins, use the Rust in Emacs. Run `` `M-x package-list-packages```, marked by` `` i``` following plug-ins installed, when all the plug-ins to choose a good after using `` `x``` perform the installation.

- Company
- Company-racer
- Racer
- Flycheck
- Flycheck-rust
- Rust-mode

The following code snippet on your `` `~ / .emacs.d / init.el``` file:

`` `
;; Enable company globally for all mode
(Global-company-mode)

;; Reduce the time after which the company auto completion popup opens
(Setq company-idle-delay 0.2)

;; Reduce the number of characters before company kicks in
(Setq company-minimum-prefix-length 1)
;; Set path to racer binary
(Setq racer-cmd "/ usr / local / bin / racer")

;; Set path to rust src directory
(Setq racer-rust-src-path "/Users/YOURUSERNAME/.rust/src/")

;; Load rust-mode when you open `.rs` files
(Add-to-list 'auto-mode-alist' ( "\\. Rs \\ '". Rust-mode))

;; Setting up configurations when you load rust-mode
(Add-hook 'rust-mode-hook

     '(Lambda ()
     ;; Enable racer
     (Racer-activate)
  
;; Hook in racer with eldoc to provide documentation
     (Racer-turn-on-eldoc)

;; Use flycheck-rust in rust-mode
     (Add-hook 'flycheck-mode-hook #' flycheck-rust-setup)

;; Use company-racer in rust mode
     (Set (make-local-variable 'company-backends)' (company-racer))

;; Key binding to jump to method definition
     (Local-set-key (kbd "M-.") # 'Racer-find-definition)

;; Key binding to auto complete and indent
     (Local-set-key (kbd "TAB") # 'racer-complete-or-indent)))

`` `

! [] (../ Image / 03-editor-emacs-error-checking.png)

## 1.3 Configuring Racer

Racer Rust need the source code for automatic completion.

- Git clone https://github.com/rust-lang/rust.git ~ / .rust
- Restart Emacs Rust and open a source code file.

! [] (../ Image / 03-editor-emacs-completion.png)

## 1.4 Conclusions

Now, you can edit the source code files in Rust in the Emacs. Features are summarized as follows:

- Syntax highlighting and auto-indentation
- Auto-completion
- Dynamic syntax error checking
- Jump to function definition
- Embedded documents

! [] (../ Image / 03-editor-emacs-jump.gif)

## Comments

1. The content of this section applies to Emacs Version 24; 23 of different configuration versions; version 22 and below is not supported.
2. MacOS comes with Emacs version 22, version 24 can be downloaded from [here] (http://emacsformacosx.com/).
