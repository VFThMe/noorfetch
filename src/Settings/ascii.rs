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

        // Список кортежей: (что ищем в строке, какой результат возвращаем)
        // Порядок важен: более специфичные названия (Lubuntu) ставим выше общих (Ubuntu)
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
pub fn ascii_art(&self) -> &'static str {
        match self {
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
            Distro::MacOS => r#"        .:'
    __ :'__
 .'`  `-'  ``.
:          .-'
:         :
 :         `-;
  `.__.-.__.'"#,
            Distro::Ubuntu => r#"       ..;,; .,;,.
    .,lool: .ooooo,
   ;oo;:    .coool.
 ....         ''' ,l;
:oooo,            'oo.
looooc            :oo'
 '::'             ,oo:
   ,.,       .... co,
    lo:;.   :oooo; .
     ':ooo; cooooc
        '''  ''''"#,

            Distro::Arch => r#"

         /\
        /  \
       /    \
      /      \
     /   ,,   \
    /   |  |   \
   /_-''    ''-_\"#,
            Distro::Fedora => r#"     __
    /  \
 __ |_
/   |
\__/"#,
            Distro::Gentoo => r#" _-----_
(       \
\    0   \
 \        )
 /      _/
(     _-
\____-"#,
            Distro::CachyOS => r#"    ____________
   /            /  o
  /      ______/
 /      /        o
/      /
\      \
 \      \__________ o
  \               /
   \_____________/"#,
            Distro::Debian => r#"  _____
 /  __ \
|  /    |
|  \___-
-_
  --_"#,
            Distro::Unknown => r#" _      ____  ____  ____  _____ _____ _____ ____ _    
/ \  /|/  _ \/  _ \/  __\/    //  __//__ __Y   _Y \ /|
| |\ ||| / \|| / \||  \/||  __\|  \    / \ |  / | |_||
| | \||| \_/|| \_/||    /| |   |  /_   | | |  \_| | ||
\_/  \|\____/\____/\_/\_\\_/   \____\  \_/ \____|_/ \|
                                                      "#,
            Distro::EndeavourOS => r#"          /o.
        /sssso-
      /ossssssso:
    /ssssssssssso+
  /ssssssssssssssso+
//osssssssssssssso+-
 `+++++++++++++++-`"#,
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
          Distro::Bazzite => r#"    \\KK999999000009999999             
   --\++++KKKK++++++++++++++,-          
  \+++++++KKKK++++++++++==~```,,-       
  9+++++++KKKK++++++++++~~````+++--     
  9KKNNNNKKKKTKKKKKNNNNNNKKK+`````+9    
  9KKKKTTTTTTTTTTKTKKKDDDKKKK``````0    
  9+++++++KKLK+++++++++++++KKKK````0    
  0+++++++KKLK++++++++++++++TKN````0    
  0+++++++KNLK+++++++++++++++KN````0    
  0++++++~KNLK+++++++++++++++KK````9    
  0++++++~KNLK++++++++++++++KKK````9    
  9+++++++KKKK+++++++++++[KKKK+````9    
  9+++++++KKKN+++++++++NNNNKKK`````9    
  9++++++++KKN++++++[[[KNKKKK``````9    
   vv+++++++KNKKNNNNKKKKKKK```````-     
    vv++++==+KKKKKKKKKKK````````,,      
      +vvv,vv+~~"""""""">M>~~`--        
            v99999999999          "#,
        Distro::Manjaro => r#"||||||||| ||||
||||||||| ||||
||||      ||||
|||| |||| ||||
|||| |||| ||||
|||| |||| ||||
|||| |||| ||||"#,
            Distro::Artix => r#"            '
           'A'
          'ooo'
         'ookxo'
         `ookxxo'
       '.   `ooko'
      'ooo`.   `oo'
     'ooxxxoo`.   `'
    'ookxxxkooo.`   .
   'ookxxkoo'`   .'oo'
  'ooxoo'`     .:ooxxo'
 'io'`             `'oo'
'`                     `'"#,
        Distro::Void => r#"    _______
 _ \______ -
| \  ___  \ |
| | /   \ | |
| | \___/ | |
| \______ \_|
 -_______\"#,
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
        Distro::Guix => r#"|.__          __.|
|__ \        / __|
   \ \      / /
    \ \    / /
     \ \  / /
      \ \/ /
       \__/"#,
        Distro::Kali => r#"     -#. #
      @###
  -######
 @#########
=##.  .#####
##      ## ##
##       ## #
##       @###
##.        ###
 ##%       ##-
  -##%    -*
   :*##+
     :*#*
       -#
        @
        :"#,
        Distro::OpenSUSE => r#" .oooo.
o   o  o
ooooo  oo
o      oo
 'oooooooooooo.
       oo      o
       oo  ooooo
        o  o   o
         'oooo'"#,
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
        Distro::Vanilla => r#"      ,x.
     ;&?^.
.-e~^+7'  )adbx,
 \#\.  `,*~ ~*/
  `~*+-'-<ay,^ 
  ,/  ,%\ `\&,
  !&UP*  +./%?"#,
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
        Distro::RedHat => r#"      .M.:MMM
     MMMMMMMMMM.
    ,MMMMMMMMMMM
 .MM MMMMMMMMMMM
MMMM   MMMMMMMMM
MMMMMM           MM
 MMMMMMMMM     ,MMMM
   MMMMMMMMMMMMMMMM:
      `MMMMMMMMMMMM "#,
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
        Distro::PopOS => r#"             /////////////
         /////////////////////
      ///////*767////////////////
    //////7676767676*//////////////
   /////76767//7676767//////////////
  /////767676///*76767///////////////
 ///////767676///76767.///7676*///////
/////////767676//76767///767676////////
//////////76767676767////76767/////////
///////////76767676//////7676//////////
////////////,7676,///////767///////////
/////////////*7676///////76////////////
///////////////7676////////////////////
 ///////////////7676///767////////////
  //////////////////////'////////////
   //////.7676767676767676767,//////
    /////767676767676767676767/////
      ///////////////////////////
         /////////////////////
             /////////////"#,
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
        Distro::FreeBSD => r#"/\,-'''''-,/\
\_)       (_/
|           |
|           |
 ;         ;
  '-_____-'"#,
        Distro::NetBSD => r#" \\`-______,----__
  \\        __,---`_
   \\       `.____
    \\-______,----`-
     \\
      \\
       \\
        \\"#,
        Distro::OpenBSD => r#"      _____
    \-     -/
 \_/         \
 |        O O |
 |_  <   )  3 )
 / \         /
    /-_____-\"#,
        }
    }

}
