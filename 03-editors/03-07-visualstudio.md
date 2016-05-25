# Visual Studio
This article is the use of VisualRust and VS GDB Debugger / VisualGDB completed in VisualStudio, edit, and debug Rust program.

## 7.1 Installation Rust, Cargo

First need to download Rust, Download https://www.rust-lang.org/downloads.html

Here we must under windows GNU ABI version because we use GDB to debug.
! [] (../ Image / 03-editor-visualstudio-download.png)

Also, you need to install Visual Studio2013 or 2015 machine.
After installing Rust, open the command line,
cargo install racer
! [] (../ Image / 03-editor-visualstudio-racer.png)

Racer Rust is used to make auto-complete, it will be used in VisualRust. Here we use rust compiled racer, do not use the built-in VisualRust racer, because it is too old.
Also Rust need to download the source code, set
RUST_SRC_PATH to Rust source src directory
! [] (../ Image / 03-editor-visualstudio-racersc.png)

## 7.2 Installation VisualRust and VS GDB Debugger

To complete the work, you can install VisualRust and VS GDB Debugger, download it here
https://github.com/PistonDevelopers/VisualRust
https://visualstudiogallery.msdn.microsoft.com/35dbae07-8c1a-4f9d-94b7-bac16cad9c01

VisualGDB can be purchased here
http://www.visualgdb.com/

Rust ## 7.3 Compile Project

New Rust Project
! [] (../ Image / 03-editor-visualstudio-newproject.png)
Setting racer and rust_src_path in tool, option in
! [] (../ Image / 03-editor-visualstudio-settings.png)
This time we can write the code can be automatically prompted. Like this
! [] (../ Image / 03-editor-visualstudio-autocomplete.png)

## 7.4 with VS GDB Debugger to debug Rust project

ok, start your Rust pleasant journey. The following start using VS GDB Debugger to debug Rust.


In the solution, add GDB debugging project
! [] (../ Image / 03-editor-visualstudio-GDBproject.png)

Directory and file name to be debugged program is located
! [] (../ Image / 03-editor-visualstudio-GDBproject-settings.png)

Setup To debug compile command used here rustc, can also be used to compile cargo
! [] (../ Image / 03-editor-visualstudio-GDBproject-settings2.png)

Add the need to debug the program's source code to the project directory
! [] (../ Image / 03-editor-visualstudio-add-files.png)

Open source file and set breakpoints information, the project is set to start the project and select Local GDB to begin debugging
! [] (../ Image / 03-editor-visualstudio-set-breakpoints.png)

! [] (../ Image / 03-editor-visualstudio-debugging2.png)


## 7.5 Debugging Rust project with VisualGDB


Build End Rust program, click debug, select the quick debug with gdb
! [] (../ Image / 03-editor-visualstudio-quickdebug.png)

Then select MingW inside and exe path

! [] (../ Image / 03-editor-visualstudio-setdebugger.png)

 Click Debug, Start Debugging your life bar

! [] (../ Image / 03-editor-visualstudio-debugging.png)
