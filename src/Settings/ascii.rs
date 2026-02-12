// Создаем список ОС
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distro {
   /* Windows,*/ MacOS, Ubuntu, Arch, Fedora, Debian, Gentoo, CachyOS,
    EndeavourOS, Trisquel, NixOS, Bazzite, Manjaro, Artix, Void,
    ALT, Guix, Kali, OpenSUSE, Lubuntu, Xubuntu, Vanilla, Garuda,
    Deepin, Nobara, Tails, RedHat, Calculate, Devuan, CentOS,
    ElementaryOS, PopOS, FreeBSD, NetBSD, OpenBSD, Unknown,
}

impl Distro {
    pub fn from_string(name: &str) -> Self {
        let name = name.to_lowercase();

        const RULES: &[(&str, Distro)] = &[
 //           ("windows", Distro::Windows),
            ("darwin", Distro::MacOS),
            ("macos", Distro::MacOS),
            ("lubuntu", Distro::Lubuntu),
            ("xubuntu", Distro::Xubuntu),
            ("ubuntu", Distro::Ubuntu),
            ("arch", Distro::Arch),
            ("fedora", Distro::Fedora),
            ("debian", Distro::Debian),
            ("gentoo", Distro::Gentoo),
            ("cachyos", Distro::CachyOS),
            ("endeavouros", Distro::EndeavourOS),
            ("trisquel", Distro::Trisquel),
            ("nixos", Distro::NixOS),
            ("bazzite", Distro::Bazzite),
            ("manjaro", Distro::Manjaro),
            ("artix", Distro::Artix),
            ("void", Distro::Void),
            ("alt linux", Distro::ALT),
            ("guix", Distro::Guix),
            ("kali", Distro::Kali),
            ("opensuse", Distro::OpenSUSE),
            ("vanilla", Distro::Vanilla),
            ("garuda", Distro::Garuda),
            ("deepin", Distro::Deepin),
            ("nobara", Distro::Nobara),
            ("tails", Distro::Tails),
            ("rhel", Distro::RedHat),
            ("red hat", Distro::RedHat),
            ("calculate", Distro::Calculate),
            ("devuan", Distro::Devuan),
            ("centos", Distro::CentOS),
            ("elementary", Distro::ElementaryOS),
            ("pop_os", Distro::PopOS),
            ("popos", Distro::PopOS),
            ("pop os", Distro::PopOS),
            ("freebsd", Distro::FreeBSD),
            ("netbsd", Distro::NetBSD),
            ("openbsd", Distro::OpenBSD),
        ];

        // Ищем первое совпадение
        RULES.iter()
            .find(|(key, _)| name.contains(key))
            .map(|(_, distro)| *distro)
            .unwrap_or(Distro::Unknown)
    }    
// Возвращаем ASCII арт для каждого дистрибутива
    pub fn ascii_art(&self) -> String {
	
	const G: &str = "\x1b[32m";
	const J: &str = "\x1b[36m";
        const Y: &str = "\x1b[33m";
        const O: &str = "\x1b[38;5;208m";
        const R: &str = "\x1b[31m";
        const M: &str = "\x1b[35m";
        const B: &str = "\x1b[34m";
        const RESET: &str = "\x1b[0m";
	
        let art = match self {
           /* Distro::Windows => r#"/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////

/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////"#,*/
            Distro::MacOS => r#"{G}        .:'
    __ :'__{RESET}
{Y} .'`  `-'  ``.\{RESET}
{O}:          .-'{RESET}
{R}:         :{RESET}
{M}:         `-;:{RESET}
{B}  `.__.-.__.'{RESET}"#,
            Distro::Ubuntu => r#"{O}       ..;,; .,;,.
    .,lool: .ooooo,
   ;oo;:    .coool.
{O} ....         ''' ,l;
{O}:oooo,            'oo.
{O}looooc            :oo'
{O} '::'             ,oo:
{O}   ,.,       .... co,
{O}    lo:;.   :oooo; .
{O}     ':ooo; cooooc
{O}        '''  ''''{RESET}"#,

            Distro::Arch => r#"
{M}         /\
{M}        /  \
{M}       /    \
{B}      /      \
{B}     /   ,,   \
{B}    /   |  |   \
{B}   /_-''    ''-_\{RESET}"#,
            Distro::Fedora => r#"

{B}     __
    /  \
{B} __ |_
{B}/   |
{B}\__/{RESET}"#,
            Distro::Gentoo => r#"{M} _-----_
{M}(       \
{M}\    0   \
{M}\        )
{M} /      _/
{M}(     _-
{M}\____-{RESET}"#,
            Distro::CachyOS => r#"{G}    ____________
   /            /  o
  /      ______/
{G} /      /        o
{G}/      /
{G}\      \
{G} \      \__________ o
{G}  \               /
{G}   \_____________/{RESET}"#,
            Distro::Debian => r#"{R}  _____
 / {RESET} _{R}_ \
{R}| {RESET} /{R}    |
{R}{RESET}|  \{R}_{RESET}{R}__-
{RESET}-{R}_
{RESET}  -{R}-_{RESET}"#,
            Distro::Unknown => r#"


{R} _      ____  ____  ____  _____ _____ _____ ____ _    
{RESET}{O}/ \  /|/  _ \/  _ \/  __\/    //  __//__ __Y   _Y \ /|{RESET}
{Y}| |\ ||| / \|| / \||  \/||  __\|  \    / \ |  / | |_||{RESET}
{G}| | \||| \_/|| \_/||    /| |   |  /_   | | |  \_| | ||{RESET}
{B}\_/  \|\____/\____/\_/\_\\_/   \____\  \_/ \____|_/ \|{RESET}
                                                      "#,
            Distro::EndeavourOS => r#"

            {M}/o.{RESET}
         {O}/{RESET}{M}sssso{RESET}{B}-{RESET}
        {O}/{RESET}{M}ossssssso{RESET}{B}:{RESET}
     {O}/{RESET}{M}sssssssssss{RESET}{B}o+{RESET}
   {O}/{RESET}{M}ssssssssssssssso{RESET}{B}+{RESET}
  {O}//{RESET}{M}osssssssssssssso{RESET}{B}+-{RESET}
  {B} `+++++++++++++++-`{RESET}"#,
          Distro::Trisquel => r#"                           ..           
                        <<!OOOO~        
                     !!mm`    ;;.       
                    ac`        ;?       
            ...     !c ..!   .;O`       
      .. fffMMMMf`. `XX!aa  ..?`        
     .DDDDDDDDfMMMMff   .  fCC          
    CDD`     ..  fMMMf   fff`           
  .C!.   !6666     MMM`fMf..            
  .C .  !!   !h    MMMMMM`              
  .L    !    .h     MMMM`    ....       
  .??..      .^     MMf    66;;!JJJ     
   `??? ..  .'^    .fRf  666      RR    
    `?;;;;;;''     .fRf  6`        R.   
       ...         .!Rf  !hh !!    ^.   
                    !fCC   mm`     ^.   
                     `CCC!      o;;^    
                        CC``;;;!o       
                           ...    "#,
          Distro::NixOS => r#"  ▗▄   ▗▄ ▄▖
 ▄▄🬸█▄▄▄🬸█▛ ▃
   ▟▛    ▜▃▟🬕
🬋🬋🬫█      █🬛🬋🬋
 🬷▛🮃▙    ▟▛
 🮃 ▟█🬴▀▀▀█🬴▀▀
  ▝▀ ▀▘   ▀▘"#,
            Distro::Bazzite => r#"{M}    \\KK999999000009999999             
   --\++++{RESET}KKKK{M}++++++++++++++,-          
  \+++++++{RESET}KKKK{M}++++++++++==~```,,-       
{M}  9+++++++{RESET}KKKK{M}++++++++++~~````+++--{RESET}     
  {M}9{RESET}KKNNNNKKKKTKKKKKNNNNNNKKK{M}+`````+9{RESET}    
  {M}9{RESET}KKKKTTTTTTTTTTKTKKKDDDKKKK{M}``````0    
  {M}9++{RESET}{M}+++++{RESET}KKLK{M}+++++++++++++{RESET}KKKK{M}````0    
  {M}0++{RESET}{M}+++++{RESET}KKLK{M}++++{RESET}{B}++++++++++{RESET}TKN{M}````0    
  {M}0{B}+++++++{RESET}KNLK{B}+++++++++++++++{RESET}KN{M}```{RESET}{M}`0    
  {M}0{RESET}{B}++++++~{RESET}KNLK{RESET}{B}+++++++++++++++{RESET}KK{B}````{RESET}{M}9    
  {M}0{RESET}{B}++++++~{RESET}KNLK{B}++++++++++++++{RESET}KKK{B}````{RESET}{M}9    
  9+++++++KKKK+++++++++++[KKKK+````9    
  9+++++++KKKN+++++++++NNNNKKK`````9    
  9++++++++KKN++++++[[[KNKKKK``````9    
   vv+++++++KNKKNNNNKKKKKKK```````-     
    vv++++==+KKKKKKKKKKK````````,,      
      +vvv,vv+~~"""""""">M>~~`--        
            v99999999999          "#,
            Distro::Manjaro => r#"

{G}||||||||| ||||
{G}||||||||| ||||
{G}||||      ||||
{G}|||| |||| ||||
{G}|||| |||| ||||
{G}|||| |||| ||||
{G}|||| |||| ||||"#,
            Distro::Artix => r#"{B}            '
{B}           'A'
{B}          'ooo'
{B}         'ookxo'
{B}         `ookxxo'
{B}       '.   `ooko'
{B}      'ooo`.   `oo'
{B}     'ooxxxoo`.   `'
{B}    'ookxxxkooo.`   .
{B}   'ookxxkoo'`   .'oo'
{B}  'ooxoo'`     .:ooxxo'
{B} 'io'`             `'oo'
'`                     `'"#,
            Distro::Void => r#"

{G}    _______
{G} _ \______ -
{G}| \  ___  \ |
{G}| | /   \ | |
{G}| | \___/ | |
{G}| \______ \_|
{G} -_______\"#,
        Distro::ALT => r#"   ``````````````````````````````````  
    ``````````````````````````````````  
    ``````````````````````````````````  
           .    .    `````````````````  
     ___   |   _/_   `````````````````  
        `  |    |    `````````````````  
   |    |  |    |    `````````````````  
   `.__/| /\__  \__/ `````````````````  
                     `````````````````  
    ```  .           `````````````````  
    ```  |   ` , __   ,   . _  .- ````  
    ```  |   | |'  `. |   |  \,'  ````  
    ```  |   | |    | |   |  /\   ````  
    ``` /\__ / /    | `._/| /  \  ````  
    ```                           ````  
    ```                           ````"#,
            Distro::Guix => r#"

  {Y}|.__          __.|
  {Y}|__ \        / __|
  {Y}   \ \      / /
  {Y}    \ \    / /
  {Y}     \ \  / /
  {Y}      \ \/ /
  {Y}       \__/"#,
            Distro::Kali => r#"
     -#. #
      @###
{B}  -######
{B} @#########
{B}=##. {RESET} .#####
{B}##     {RESET} ## ##
{B}##       {RESET}## #
{B}##       {RESET}@###
{B}##.        {RESET}###
{B}##%       {RESET}##-
{B}  -##%{RESET}    -*
{B}   :*##+
{B}     :*#*
{B}       -#
{B}        @
{B}        :"#,
            Distro::OpenSUSE => r#"

{G} .oooo.
{G}o   o  o
{G}ooooo  oo
{G}o      oo
{G} 'oooooooooooo.
{G}       oo      o
{G}       oo  ooooo
{G}        o  o   o
{G}         'oooo'"#,
        Distro::Lubuntu => r#"                 ...........           
                `77MMMMMMMMMMM``        
               hhMM``      `vMM======   
              !hhTT         `d>===`     
    MMh'      !hhT                      
    MMhhhh    !h?                       
    MM`  hhh    ?            I          
    MMM    ffn`              Y'         
     MM      ffn             Yj         
      M                      jj         
      Mn                      jW'       
  MMMMMMnnnn`                  WW       
  `MMMoooonnnnnn`              WW'      
   `MM`     `ooonn             WMM      
     MM`                        NN      
      MMp`                     NN'      
       MMpvv`               ooNN        
          `vvvvv'          ggg'"#,
        Distro::Xubuntu => r#"             __ygg@@@@@@@@@ggy__
         _yg@@@@@@@@@@@@@@@@@@@@@gy_
      _a@@@@@@@@@@@@@@@@@@@@@@@@@@@@@y_
    _a@@@@@@@@@@@@@@@@@@@@@@@#@@@@@@@@@g_
   a@@@@@@@@@@@@@###@@@@@@@@##@@@@##@@@@@k
  g@@@@@@@###@@@#####@@@@@@@##@@###@@@@@@@k
 a@@@@@@@@#####@#####@@@@@@##@@###@@@@@@@@@k
j@@@@@@@@@############@@@@@##@###@@@@@@@@@@@k
g@@@@@@@@@#####################@@@@@@@@@@@@@@
@@@@@@@@@##########################@@@@@@@@@@
0@@@@@@@@###########################@@@@@@@@@
~@@@@@@@############################@@@@@@@@F
 9@@@@@@##########################@@@@@@@@@P
  4@@@@@@######################@@@@@@@@@@@P
   ~@@@@@@################@@@@@@@@@@@@@@@F
    `4@@@@@@#######@@@@@@@@@@@@@@@@@@@@P`
      `~@@@@@@@@@@@@@@@@@@@@@@@@@@@@@F`
         ~~4@@@@@@@@@@@@@@@@@@@@@P~~
             `~~=R@@@@@@@@@P=~~~"#,
            Distro::Vanilla => r#"

{Y}      ,x.
{Y}     ;&?^.
{Y}.-e~^+7'  )adbx,
{Y} \#\.  `,*~ ~*/
{Y}  `~*+-'-<ay,^ 
{Y}  ,/  ,%\ `\&,
{Y}  !&UP*  +./%?"#,
        Distro::Garuda => r#"     .----.
   .'   ,  '.
 .'    '-----|
'.   -----,
  '.____.'"#,
        Distro::Nobara => r#"    ...           """""""""             
  f*00000*     ((@@@@@@@@@@@))          
 .0000000*! JJJMMMMMMMMMMMMMM@LLLL``    
 .000000mm?JJJJMMMMMMMMMMMMMMMMMMMM@``  
 .000000mm?JJJJ@@MMMMMMMMMMMMMMMMMMMMMt 
 .000000mmTJXJJJJ```DDDDDDMMMMMMMMMMMMM 
 .00000mmmmJXJJ`        0DDDMMMMMMMMMMM 
 .00000mmmmJXJ`           0DJMMMMMMMMMM 
 .00000mmJmJfJ   @@@        JJMMMMMMMMM 
 .00000mmJmJf`    @@@@@@@@  `JMMMMMMMMM 
 .00000mmJmJf   @@ @@@       aaaaa`MMMM 
 .000000mJmJf   @@@@@@           ---MMM 
 .000000mmmJf                 X``MMMMMM 
 .0000000mmqff                XHHMMMMMM 
 .0000000mmqqqff              XHHMMMMMM 
 .0000000mm0qqqqq             XHHMMMMMM 
 .0000000mm0qqqqqq            @HHMMMMMM 
 .0000000mm` ..qqq            @@HMMMMMM 
  f0000000u    .qq            ``@@@@@@@ 
    .000`       .q                ````` 
"#,
        Distro::Tails => r#"      ``
  ./yhNh
syy/Nshh         `:o/
N:dsNshh  █   `ohNMMd
N-/+Nshh      `yMMMMd
N-yhMshh       yMMMMd
N-s:hshh  █    yMMMMd so//.
N-oyNsyh       yMMMMd d  Mms.
N:hohhhd:.     yMMMMd  syMMM+
Nsyh+-..+y+-   yMMMMd   :mMM+
+hy-      -ss/`yMMMM     `+d+
  :sy/.     ./yNMMMMm      ``
    .+ys- `:+hNMMMMMMy/`
      `hNmmMMMMMMMMMMMMdo.
       dMMMMMMMMMMMMMMMMMNh:
       +hMMMMMMMMMMMMMMMMMmy.
         -oNMMMMMMMMMMmy+.`
           `:yNMMMds/.`
              .//`"#,
            Distro::RedHat => r#"
{R}      .M.:MMM
{R}     MMMMMMMMMM.
{R}    ,MMMMMMMMMMM
{R} .MM MMMMMMMMMMM
{R}MMMM   MMMMMMMMM
{R}MMMMMM           MM
{R} MMMMMMMMM     ,MMMM
{R}   MMMMMMMMMMMMMMMM:
{R}      `MMMMMMMMMMMM "#,
        Distro::Calculate => r#"                              ......
                           ,,+++++++,.
                         .,,,....,,,+**+,,.
                       ............,++++,,,
                      ...............
                    ......,,,........
                  .....+*#####+,,,*+.
              .....,*###############,..,,,,,,..
           ......,*#################*..,,,,,..,,,..
         .,,....*####################+***+,,,,...,++,
       .,,..,..*#####################*,
     ,+,.+*..*#######################.
   ,+,,+*+..,########################*
.,++++++.  ..+##**###################+
.....      ..+##***#################*.
           .,.*#*****##############*.
           ..,,*********#####****+.
     .,++*****+++*****************+++++,.
      ,++++++**+++++***********+++++++++,
     .,,,,++++,..  .,,,,,.....,+++,.,,"#,
        Distro::CentOS => r#" ____^____
 |\  |  /|
 | \ | / |
<---- ---->
 | / | \ |
 |/__|__\|
     v"#,
        Distro::ElementaryOS => r#"         eeeeeeeeeeeeeeeee
      eeeeeeeeeeeeeeeeeeeeeee
    eeeee  eeeeeeeeeeee   eeeee
  eeee   eeeee       eee     eeee
 eeee   eeee          eee     eeee
eee    eee            eee       eee
eee   eee            eee        eee
ee    eee           eeee       eeee
ee    eee         eeeee      eeeeee
ee    eee       eeeee      eeeee ee
eee   eeee   eeeeee      eeeee  eee
eee    eeeeeeeeee     eeeeee    eee
 eeeeeeeeeeeeeeeeeeeeeeee    eeeee
  eeeeeeee eeeeeeeeeeee      eeee
    eeeee                 eeeee
      eeeeeee         eeeeeee
         eeeeeeeeeeeeeeeee"#,
            Distro::PopOS => r#"
{B}             /////////////
{B}          /////////////////////
{B}       ///////{RESET}767{B}////////////////
{B}    //////{RESET}7676767676{B}*//////////////
{B}   /////{RESET}76767{B}//{RESET}7676767{B}//////////////
{B}  /////{RESET}767676{B}///*{RESET}76767{B}///////////////
{B} ///////{RESET}767676{B}///{RESET}76767{B}.///{RESET}7676{B}*///////
{B} /////////{RESET}767676{B}//{RESET}76767{B}///{RESET}767676{B}////////
{B} //////////{RESET}76767676767{B}////{RESET}76767{B}/////////
{B} ///////////{RESET}76767676{B}//////{RESET}7676{B}//////////
{B} ////////////,{RESET}7676{B},///////{RESET}767{B}///////////
{B} /////////////{B}*{RESET}7676{B}///////{RESET}76{B}////////////
{B} ///////////////{RESET}7676{B}////////////////////
{B} ///////////////{RESET}7676{B}///{RESET}767{B}////////////
{B}  //////////////////////'////////////
{B}   //////.{RESET}7676767676767676767{B},//////
{B}     /////{RESET}767676767676767676767{B}/////
{B}       ///////////////////////////
{B}          /////////////////////
{B}              /////////////"#,
        Distro::Devuan => r#"    ..-==-
        .+#:
         =@@
      :+%@#:
.:=+#@@%*:
#@@@#=:#,"#,
        Distro::Deepin => r#"             ............
         .';;;;;.       .,;,.
      .,;;;;;;;.       ';;;;;;;.
    .;::::::::'     .,::;;,''''',.
   ,'.::::::::    .;;'.          ';
  ;'  'cccccc,   ,' :: '..        .:
 ,,    :ccccc.  ;: .c, '' :.       ,;
.l.     cllll' ., .lc  :; .l'       l.
.c       :lllc  ;cl:  .l' .ll.      :'
.l        'looc. .   ,o:  'oo'      c,
.o.         .:ool::coc'  .ooo'      o.
 ::            .....   .;dddo      ;c
  l:...            .';lddddo.     ,o
   lxxxxxdoolllodxxxxxxxxxc      :l
    ,dxxxxxxxxxxxxxxxxxxl.     'o,
      ,dkkkkkkkkkkkkko;.    .;o;
        .;okkkkkdl;.    .,cl:.
            .,:cccccccc:,."#,
            Distro::FreeBSD => r#"


 {R}/\,-'''''-,/\
 {R}\_)       (_/
 {R}|           |
 {R}|           |
 {R} ;         ;
 {R}  '-_____-'"#,
        Distro::NetBSD => r#" \\`-______,----__
  \\        __,---`_
   \\       `.____
    \\-______,----`-
     \\
      \\
       \\
        \\"#,
            Distro::OpenBSD => r#"

{Y}      _____
{Y}    \-     -/
{Y} \_/         \
{Y} |        O O |
{Y} |_  <   )  3 )
{Y} / \         /
{Y}    /-_____-\"#,
        };
	art.replace("{G}", G)
            .replace("{Y}", Y)
            .replace("{O}", O)
            .replace("{R}", R)
            .replace("{M}", M)
            .replace("{B}", B)
	    .replace("J", J)
            .replace("{RESET}", RESET)
    }

}
