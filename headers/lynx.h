#ifndef __LYNX_H__
#define __LYNX_H__

const unsigned char BLOCKSIZE = DBLOCKSIZE;

zp unsigned char cctmp;          
zp unsigned char __iodat;        
zp unsigned char __iodir;        
zp unsigned char FileStartBlock; 
zp unsigned short FileBlockOffset;
zp unsigned short FileDestAddr;   
zp unsigned short FileFileLen;    
zp unsigned char FileExecFlag;   
zp unsigned char FileCurrBlock;  
zp unsigned short FileBlockByte;  
zp unsigned short FileDestPtr;    

unsigned char * const TMPADRL = 0xFC00;
unsigned char * const TMPADRH = 0xFC01;
unsigned char * const TILTACUML = 0xFC02;
unsigned char * const TILTACUMH = 0xFC03;
unsigned char * const HOFFL = 0xFC04;
unsigned char * const HOFFH = 0xFC05;
unsigned char * const VOFFL = 0xFC06;
unsigned char * const VOFFH = 0xFC07;
unsigned char * const VIDBASL = 0xFC08;
unsigned char * const VIDBASH = 0xFC09;
unsigned char * const COLLBASL = 0xFC0A;
unsigned char * const COLLBASH = 0xFC0B;
unsigned char * const VIDADRL = 0xFC0C;
unsigned char * const VIDADRH = 0xFC0D;
unsigned char * const COLLADRL = 0xFC0E;
unsigned char * const COLLADRH = 0xFC0F;
unsigned char * const SCBNEXTL = 0xFC10;
unsigned char * const SCBNEXTH = 0xFC11;
unsigned char * const SPRDLINEL = 0xFC12;
unsigned char * const SPRDLINEH = 0xFC13;
unsigned char * const HPOSSTRTL = 0xFC14;
unsigned char * const HPOSSTRTH = 0xFC15;
unsigned char * const VPOSSTRTL = 0xFC16;
unsigned char * const VPOSSTRTH = 0xFC17;
unsigned char * const SPRHSIZL = 0xFC18;
unsigned char * const SPRHSIZH = 0xFC19;
unsigned char * const SPRVSIZL = 0xFC1A;
unsigned char * const SPRVSIZH = 0xFC1B;
unsigned char * const STRETCHL = 0xFC1C;
unsigned char * const STRETCHH = 0xFC1D;
unsigned char * const TILTL = 0xFC1E;
unsigned char * const TILTH = 0xFC1F;
unsigned char * const SPRDOFFL = 0xFC20;
unsigned char * const SPRDOFFH = 0xFC21;
unsigned char * const SPRVPOSL = 0xFC22;
unsigned char * const SPRVPOSH = 0xFC23;
unsigned char * const COLLOFFL = 0xFC24;
unsigned char * const COLLOFFH = 0xFC25;
unsigned char * const VSIZACUML = 0xFC26;
unsigned char * const VSIZACUMH = 0xFC27;
unsigned char * const HSIZOFFL = 0xFC28;
unsigned char * const HSIZOFFH = 0xFC29;
unsigned char * const VSIZOFFL = 0xFC2A;
unsigned char * const VSIZOFFH = 0xFC2B;
unsigned char * const SCBADRL = 0xFC2C;
unsigned char * const SCBADRH = 0xFC2D;
unsigned char * const PROCADRL = 0xFC2E;
unsigned char * const PROCADRH = 0xFC2F;

unsigned char * const MATHD = 0xFC52;
unsigned char * const MATHC = 0xFC53;
unsigned char * const MATHB = 0xFC54;
unsigned char * const MATHA = 0xFC55;
unsigned char * const MATHP = 0xFC56;
unsigned char * const MATHN = 0xFC57;
unsigned char * const MATHH = 0xFC60;
unsigned char * const MATHG = 0xFC61;
unsigned char * const MATHF = 0xFC62;
unsigned char * const MATHE = 0xFC63;
unsigned char * const MATHM = 0xFC6C;
unsigned char * const MATHL = 0xFC6D;
unsigned char * const MATHK = 0xFC6E;
unsigned char * const MATHJ = 0xFC6F;

unsigned char * const SPRCTL0 = 0xFC80;
unsigned char * const SPRCTL1 = 0xFC81;
unsigned char * const SPRCOLL = 0xFC82;
unsigned char * const SPRINIT = 0xFC83;
unsigned char * const SUZYHREV = 0xFC88;
unsigned char * const SUZYSREV = 0xFC89;
unsigned char * const SUZYBUSEN = 0xFC90;
unsigned char * const SPRGO = 0xFC91;
unsigned char * const SPRSYS = 0xFC92;
unsigned char * const JOYSTICK = 0xFCB0;
unsigned char * const SWITCHES = 0xFCB1;
unsigned char * const RCART0 = 0xFCB2;
unsigned char * const RCART1 = 0xFCB3;
unsigned char * const LEDS = 0xFCC0;
unsigned char * const PARSTATUS = 0xFCC2;
unsigned char * const PARDATA = 0xFCC3;
unsigned char * const HOWIE = 0xFCC4;

unsigned char * const TIMER0 = 0xFD00;
unsigned char * const TIMER1 = 0xFD04;
unsigned char * const TIMER2 = 0xFD08;
unsigned char * const TIMER3 = 0xFD0C;
unsigned char * const TIMER4 = 0xFD10;
unsigned char * const TIMER5 = 0xFD14;
unsigned char * const TIMER6 = 0xFD18;
unsigned char * const TIMER7 = 0xFD1C;
unsigned char * const HTIMER = 0xFD00;
unsigned char * const VTIMER = 0xFD08;

unsigned char * const HTIMBKUP = 0xFD00;
unsigned char * const HTIMCTLA = 0xFD01;
unsigned char * const HTIMCNT = 0xFD02;
unsigned char * const HTIMCTLB = 0xFD03;
unsigned char * const VTIMBKUP = 0xFD08;
unsigned char * const VTIMCTLA = 0xFD09;
unsigned char * const VTIMCNT = 0xFD0A;
unsigned char * const VTIMCTLB = 0xFD0B;
unsigned char * const BAUDBKUP = 0xFD10;

unsigned char * const TIM0BKUP = 0xFD00;
unsigned char * const TIM0CTLA = 0xFD01;
unsigned char * const TIM0CNT = 0xFD02;
unsigned char * const TIM0CTLB = 0xFD03;
unsigned char * const TIM1BKUP = 0xFD04;
unsigned char * const TIM1CTLA = 0xFD05;
unsigned char * const TIM1CNT = 0xFD06;
unsigned char * const TIM1CTLB = 0xFD07;
unsigned char * const TIM2BKUP = 0xFD08;
unsigned char * const TIM2CTLA = 0xFD09;
unsigned char * const TIM2CNT = 0xFD0A;
unsigned char * const TIM2CTLB = 0xFD0B;
unsigned char * const TIM3BKUP = 0xFD0C;
unsigned char * const TIM3CTLA = 0xFD0D;
unsigned char * const TIM3CNT = 0xFD0E;
unsigned char * const TIM3CTLB = 0xFD0F;
unsigned char * const TIM4BKUP = 0xFD10;
unsigned char * const TIM4CTLA = 0xFD11;
unsigned char * const TIM4CNT = 0xFD12;
unsigned char * const TIM4CTLB = 0xFD13;
unsigned char * const TIM5BKUP = 0xFD14;
unsigned char * const TIM5CTLA = 0xFD15;
unsigned char * const TIM5CNT = 0xFD16;
unsigned char * const TIM5CTLB = 0xFD17;
unsigned char * const TIM6BKUP = 0xFD18;
unsigned char * const TIM6CTLA = 0xFD19;
unsigned char * const TIM6CNT = 0xFD1A;
unsigned char * const TIM6CTLB = 0xFD1B;
unsigned char * const TIM7BKUP = 0xFD1C;
unsigned char * const TIM7CTLA = 0xFD1D;
unsigned char * const TIM7CNT = 0xFD1E;
unsigned char * const TIM7CTLB = 0xFD1F;

unsigned char * const AUDIO0 = 0xFD20;
unsigned char * const AUDIO1 = 0xFD28;
unsigned char * const AUDIO2 = 0xFD30;
unsigned char * const AUDIO3 = 0xFD38;

unsigned char * const AUD0VOL = 0xFD20;
unsigned char * const AUD0FEED = 0xFD21;
unsigned char * const AUD0OUT = 0xFD22;
unsigned char * const AUD0SHIFT = 0xFD23;
unsigned char * const AUD0BKUP = 0xFD24;
unsigned char * const AUD0CTLA = 0xFD25;
unsigned char * const AUD0CNT = 0xFD26;
unsigned char * const AUD0CTLB = 0xFD27;
unsigned char * const AUD1VOL = 0xFD28;
unsigned char * const AUD1FEED = 0xFD29;
unsigned char * const AUD1OUT = 0xFD2A;
unsigned char * const AUD1SHIFT = 0xFD2B;
unsigned char * const AUD1BKUP = 0xFD2C;
unsigned char * const AUD1CTLA = 0xFD2D;
unsigned char * const AUD1CNT = 0xFD2E;
unsigned char * const AUD1CTLB = 0xFD2F;
unsigned char * const AUD2VOL = 0xFD30;
unsigned char * const AUD2FEED = 0xFD31;
unsigned char * const AUD2OUT = 0xFD32;
unsigned char * const AUD2SHIFT = 0xFD33;
unsigned char * const AUD2BKUP = 0xFD34;
unsigned char * const AUD2CTLA = 0xFD35;
unsigned char * const AUD2CNT = 0xFD36;
unsigned char * const AUD2CTLB = 0xFD37;
unsigned char * const AUD3VOL = 0xFD38;
unsigned char * const AUD3FEED = 0xFD39;
unsigned char * const AUD3OUT = 0xFD3A;
unsigned char * const AUD3SHIFT = 0xFD3B;
unsigned char * const AUD3BKUP = 0xFD3C;
unsigned char * const AUD3CTLA = 0xFD3D;
unsigned char * const AUD3CNT = 0xFD3E;
unsigned char * const AUD3CTLB = 0xFD3F;
unsigned char * const MSTEREO = 0xFD50;

#define TIMER0_INTERRUPT 0x01
#define TIMER1_INTERRUPT 0x02
#define TIMER2_INTERRUPT 0x04
#define TIMER3_INTERRUPT 0x08
#define TIMER4_INTERRUPT 0x10
#define TIMER5_INTERRUPT 0x20
#define TIMER6_INTERRUPT 0x40
#define TIMER7_INTERRUPT 0x80

#define HBL_INTERRUPT 0x01
#define VBL_INTERRUPT 0x04
#define SERIAL_INTERRUPT 0x10

unsigned char * const INTRST = 0xFD80;
unsigned char * const INTSET = 0xFD81;
unsigned char * const MAGRDY0 = 0xFD84;
unsigned char * const MAGRDY1 = 0xFD85;
unsigned char * const AUDIN = 0xFD86;
unsigned char * const SYSCTL1 = 0xFD87;
unsigned char * const MIKEYHREV = 0xFD88;
unsigned char * const MIKEYSREV = 0xFD89;
unsigned char * const IODIR = 0xFD8A;
unsigned char * const IODAT = 0xFD8B;

#define TxIntEnable 0b10000000
#define RxIntEnable 0b01000000
#define TxParEnable 0b00010000
#define ResetErr 0b00001000
#define TxOpenColl 0b00000100
#define TxBreak 0b00000010
#define ParEven 0b00000001
#define TxReady 0b10000000
#define RxReady 0b01000000
#define TxEmpty 0b00100000
#define RxParityErr 0b00010000
#define RxOverrun 0b00001000
#define RxFrameErr 0b00000100
#define RxBreak 0b00000010
#define ParityBit 0b00000001

unsigned char * const SERCTL = 0xFD8C;
unsigned char * const SERDAT = 0xFD8D;
unsigned char * const SDONEACK = 0xFD90;
unsigned char * const CPUSLEEP = 0xFD91;
unsigned char * const DISPCTL = 0xFD92;
unsigned char * const PBKUP = 0xFD93;
unsigned char * const DISPADRL = 0xFD94;
unsigned char * const DISPADRH = 0xFD95;
unsigned char * const MTEST0 = 0xFD9C;
unsigned char * const MTEST1 = 0xFD9D;
unsigned char * const MTEST2 = 0xFD9E;
unsigned char * const PALETTE = 0xFDA0;
unsigned char * const GCOLMAP = 0xFDA0;
unsigned char * const RBCOLMAP = 0xFDB0;

unsigned char * const MAPCTL = 0xFFF9;
unsigned char * const VECTORS = 0xFFFB;
unsigned char * const INTVECTL = 0xFFFE;
unsigned char * const INTVECTH = 0xFFFF;
unsigned char * const RSTVECTL = 0xFFFC;
unsigned char * const RSTVECTH = 0xFFFD;
unsigned char * const NMIVECTL = 0xFFFA;
unsigned char * const NMIVECTH = 0xFFFB;

#endif