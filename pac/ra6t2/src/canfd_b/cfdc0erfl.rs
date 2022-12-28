#[doc = "Register `CFDC0ERFL` reader"]
pub struct R(crate::R<CFDC0ERFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDC0ERFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDC0ERFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDC0ERFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDC0ERFL` writer"]
pub struct W(crate::W<CFDC0ERFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDC0ERFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFDC0ERFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDC0ERFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BEF` reader - Bus Error Flag"]
pub type BEF_R = crate::BitReader<BEF_A>;
#[doc = "Bus Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEF_A {
    #[doc = "0: Channel bus error not detected"]
    _0 = 0,
    #[doc = "1: Channel bus error detected"]
    _1 = 1,
}
impl From<BEF_A> for bool {
    #[inline(always)]
    fn from(variant: BEF_A) -> Self {
        variant as u8 != 0
    }
}
impl BEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEF_A {
        match self.bits {
            false => BEF_A::_0,
            true => BEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEF_A::_1
    }
}
#[doc = "Field `BEF` writer - Bus Error Flag"]
pub type BEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, BEF_A, O>;
impl<'a, const O: u8> BEF_W<'a, O> {
    #[doc = "Channel bus error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEF_A::_0)
    }
    #[doc = "Channel bus error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEF_A::_1)
    }
}
#[doc = "Field `EWF` reader - Error Warning Flag"]
pub type EWF_R = crate::BitReader<EWF_A>;
#[doc = "Error Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWF_A {
    #[doc = "0: Channel error warning not detected"]
    _0 = 0,
    #[doc = "1: Channel error warning detected"]
    _1 = 1,
}
impl From<EWF_A> for bool {
    #[inline(always)]
    fn from(variant: EWF_A) -> Self {
        variant as u8 != 0
    }
}
impl EWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWF_A {
        match self.bits {
            false => EWF_A::_0,
            true => EWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWF_A::_1
    }
}
#[doc = "Field `EWF` writer - Error Warning Flag"]
pub type EWF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, EWF_A, O>;
impl<'a, const O: u8> EWF_W<'a, O> {
    #[doc = "Channel error warning not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EWF_A::_0)
    }
    #[doc = "Channel error warning detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EWF_A::_1)
    }
}
#[doc = "Field `EPF` reader - Error Passive Flag"]
pub type EPF_R = crate::BitReader<EPF_A>;
#[doc = "Error Passive Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPF_A {
    #[doc = "0: Channel error passive not detected"]
    _0 = 0,
    #[doc = "1: Channel error passive detected"]
    _1 = 1,
}
impl From<EPF_A> for bool {
    #[inline(always)]
    fn from(variant: EPF_A) -> Self {
        variant as u8 != 0
    }
}
impl EPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPF_A {
        match self.bits {
            false => EPF_A::_0,
            true => EPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPF_A::_1
    }
}
#[doc = "Field `EPF` writer - Error Passive Flag"]
pub type EPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, EPF_A, O>;
impl<'a, const O: u8> EPF_W<'a, O> {
    #[doc = "Channel error passive not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPF_A::_0)
    }
    #[doc = "Channel error passive detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPF_A::_1)
    }
}
#[doc = "Field `BOEF` reader - Bus-Off Entry Flag"]
pub type BOEF_R = crate::BitReader<BOEF_A>;
#[doc = "Bus-Off Entry Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOEF_A {
    #[doc = "0: Channel bus-off entry not detected"]
    _0 = 0,
    #[doc = "1: Channel bus-off entry detected"]
    _1 = 1,
}
impl From<BOEF_A> for bool {
    #[inline(always)]
    fn from(variant: BOEF_A) -> Self {
        variant as u8 != 0
    }
}
impl BOEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOEF_A {
        match self.bits {
            false => BOEF_A::_0,
            true => BOEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOEF_A::_1
    }
}
#[doc = "Field `BOEF` writer - Bus-Off Entry Flag"]
pub type BOEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, BOEF_A, O>;
impl<'a, const O: u8> BOEF_W<'a, O> {
    #[doc = "Channel bus-off entry not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOEF_A::_0)
    }
    #[doc = "Channel bus-off entry detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOEF_A::_1)
    }
}
#[doc = "Field `BORF` reader - Bus-Off Recovery Flag"]
pub type BORF_R = crate::BitReader<BORF_A>;
#[doc = "Bus-Off Recovery Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORF_A {
    #[doc = "0: Channel bus-off recovery not detected"]
    _0 = 0,
    #[doc = "1: Channel bus-off recovery detected"]
    _1 = 1,
}
impl From<BORF_A> for bool {
    #[inline(always)]
    fn from(variant: BORF_A) -> Self {
        variant as u8 != 0
    }
}
impl BORF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORF_A {
        match self.bits {
            false => BORF_A::_0,
            true => BORF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BORF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BORF_A::_1
    }
}
#[doc = "Field `BORF` writer - Bus-Off Recovery Flag"]
pub type BORF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, BORF_A, O>;
impl<'a, const O: u8> BORF_W<'a, O> {
    #[doc = "Channel bus-off recovery not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BORF_A::_0)
    }
    #[doc = "Channel bus-off recovery detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BORF_A::_1)
    }
}
#[doc = "Field `OVLF` reader - Overload Flag"]
pub type OVLF_R = crate::BitReader<OVLF_A>;
#[doc = "Overload Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVLF_A {
    #[doc = "0: Channel overload not detected"]
    _0 = 0,
    #[doc = "1: Channel overload detected"]
    _1 = 1,
}
impl From<OVLF_A> for bool {
    #[inline(always)]
    fn from(variant: OVLF_A) -> Self {
        variant as u8 != 0
    }
}
impl OVLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVLF_A {
        match self.bits {
            false => OVLF_A::_0,
            true => OVLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVLF_A::_1
    }
}
#[doc = "Field `OVLF` writer - Overload Flag"]
pub type OVLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, OVLF_A, O>;
impl<'a, const O: u8> OVLF_W<'a, O> {
    #[doc = "Channel overload not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVLF_A::_0)
    }
    #[doc = "Channel overload detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVLF_A::_1)
    }
}
#[doc = "Field `BLF` reader - Bus Lock Flag"]
pub type BLF_R = crate::BitReader<BLF_A>;
#[doc = "Bus Lock Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLF_A {
    #[doc = "0: Channel bus lock not detected"]
    _0 = 0,
    #[doc = "1: Channel bus lock detected"]
    _1 = 1,
}
impl From<BLF_A> for bool {
    #[inline(always)]
    fn from(variant: BLF_A) -> Self {
        variant as u8 != 0
    }
}
impl BLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLF_A {
        match self.bits {
            false => BLF_A::_0,
            true => BLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLF_A::_1
    }
}
#[doc = "Field `BLF` writer - Bus Lock Flag"]
pub type BLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, BLF_A, O>;
impl<'a, const O: u8> BLF_W<'a, O> {
    #[doc = "Channel bus lock not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLF_A::_0)
    }
    #[doc = "Channel bus lock detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLF_A::_1)
    }
}
#[doc = "Field `ALF` reader - Arbitration Lost Flag"]
pub type ALF_R = crate::BitReader<ALF_A>;
#[doc = "Arbitration Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALF_A {
    #[doc = "0: Channel arbitration lost not detected"]
    _0 = 0,
    #[doc = "1: Channel arbitration lost detected"]
    _1 = 1,
}
impl From<ALF_A> for bool {
    #[inline(always)]
    fn from(variant: ALF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALF_A {
        match self.bits {
            false => ALF_A::_0,
            true => ALF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALF_A::_1
    }
}
#[doc = "Field `ALF` writer - Arbitration Lost Flag"]
pub type ALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, ALF_A, O>;
impl<'a, const O: u8> ALF_W<'a, O> {
    #[doc = "Channel arbitration lost not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALF_A::_0)
    }
    #[doc = "Channel arbitration lost detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALF_A::_1)
    }
}
#[doc = "Field `SERR` reader - Stuff Error"]
pub type SERR_R = crate::BitReader<SERR_A>;
#[doc = "Stuff Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SERR_A {
    #[doc = "0: Channel stuff error not detected"]
    _0 = 0,
    #[doc = "1: Channel stuff error detected"]
    _1 = 1,
}
impl From<SERR_A> for bool {
    #[inline(always)]
    fn from(variant: SERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SERR_A {
        match self.bits {
            false => SERR_A::_0,
            true => SERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SERR_A::_1
    }
}
#[doc = "Field `SERR` writer - Stuff Error"]
pub type SERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, SERR_A, O>;
impl<'a, const O: u8> SERR_W<'a, O> {
    #[doc = "Channel stuff error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SERR_A::_0)
    }
    #[doc = "Channel stuff error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SERR_A::_1)
    }
}
#[doc = "Field `FERR` reader - Form Error"]
pub type FERR_R = crate::BitReader<FERR_A>;
#[doc = "Form Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FERR_A {
    #[doc = "0: Channel form error not detected"]
    _0 = 0,
    #[doc = "1: Channel form error detected"]
    _1 = 1,
}
impl From<FERR_A> for bool {
    #[inline(always)]
    fn from(variant: FERR_A) -> Self {
        variant as u8 != 0
    }
}
impl FERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FERR_A {
        match self.bits {
            false => FERR_A::_0,
            true => FERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FERR_A::_1
    }
}
#[doc = "Field `FERR` writer - Form Error"]
pub type FERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, FERR_A, O>;
impl<'a, const O: u8> FERR_W<'a, O> {
    #[doc = "Channel form error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FERR_A::_0)
    }
    #[doc = "Channel form error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FERR_A::_1)
    }
}
#[doc = "Field `AERR` reader - Acknowledge Error"]
pub type AERR_R = crate::BitReader<AERR_A>;
#[doc = "Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AERR_A {
    #[doc = "0: Channel acknowledge error not detected"]
    _0 = 0,
    #[doc = "1: Channel acknowledge error detected"]
    _1 = 1,
}
impl From<AERR_A> for bool {
    #[inline(always)]
    fn from(variant: AERR_A) -> Self {
        variant as u8 != 0
    }
}
impl AERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AERR_A {
        match self.bits {
            false => AERR_A::_0,
            true => AERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AERR_A::_1
    }
}
#[doc = "Field `AERR` writer - Acknowledge Error"]
pub type AERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, AERR_A, O>;
impl<'a, const O: u8> AERR_W<'a, O> {
    #[doc = "Channel acknowledge error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AERR_A::_0)
    }
    #[doc = "Channel acknowledge error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AERR_A::_1)
    }
}
#[doc = "Field `CERR` reader - CRC Error"]
pub type CERR_R = crate::BitReader<CERR_A>;
#[doc = "CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERR_A {
    #[doc = "0: Channel CRC error not detected"]
    _0 = 0,
    #[doc = "1: Channel CRC error detected"]
    _1 = 1,
}
impl From<CERR_A> for bool {
    #[inline(always)]
    fn from(variant: CERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERR_A {
        match self.bits {
            false => CERR_A::_0,
            true => CERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERR_A::_1
    }
}
#[doc = "Field `CERR` writer - CRC Error"]
pub type CERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, CERR_A, O>;
impl<'a, const O: u8> CERR_W<'a, O> {
    #[doc = "Channel CRC error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CERR_A::_0)
    }
    #[doc = "Channel CRC error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CERR_A::_1)
    }
}
#[doc = "Field `B1ERR` reader - Bit 1 Error"]
pub type B1ERR_R = crate::BitReader<B1ERR_A>;
#[doc = "Bit 1 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1ERR_A {
    #[doc = "0: Channel bit 1 error not detected"]
    _0 = 0,
    #[doc = "1: Channel bit 1 error detected"]
    _1 = 1,
}
impl From<B1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: B1ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl B1ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1ERR_A {
        match self.bits {
            false => B1ERR_A::_0,
            true => B1ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1ERR_A::_1
    }
}
#[doc = "Field `B1ERR` writer - Bit 1 Error"]
pub type B1ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, B1ERR_A, O>;
impl<'a, const O: u8> B1ERR_W<'a, O> {
    #[doc = "Channel bit 1 error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1ERR_A::_0)
    }
    #[doc = "Channel bit 1 error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1ERR_A::_1)
    }
}
#[doc = "Field `B0ERR` reader - Bit 0 Error"]
pub type B0ERR_R = crate::BitReader<B0ERR_A>;
#[doc = "Bit 0 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0ERR_A {
    #[doc = "0: Channel bit 0 error not detected"]
    _0 = 0,
    #[doc = "1: Channel bit 0 error detected"]
    _1 = 1,
}
impl From<B0ERR_A> for bool {
    #[inline(always)]
    fn from(variant: B0ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl B0ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0ERR_A {
        match self.bits {
            false => B0ERR_A::_0,
            true => B0ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0ERR_A::_1
    }
}
#[doc = "Field `B0ERR` writer - Bit 0 Error"]
pub type B0ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, B0ERR_A, O>;
impl<'a, const O: u8> B0ERR_W<'a, O> {
    #[doc = "Channel bit 0 error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0ERR_A::_0)
    }
    #[doc = "Channel bit 0 error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0ERR_A::_1)
    }
}
#[doc = "Field `ADERR` reader - Acknowledge Delimiter Error"]
pub type ADERR_R = crate::BitReader<ADERR_A>;
#[doc = "Acknowledge Delimiter Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADERR_A {
    #[doc = "0: Channel acknowledge delimiter error not detected"]
    _0 = 0,
    #[doc = "1: Channel acknowledge delimiter error detected"]
    _1 = 1,
}
impl From<ADERR_A> for bool {
    #[inline(always)]
    fn from(variant: ADERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADERR_A {
        match self.bits {
            false => ADERR_A::_0,
            true => ADERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADERR_A::_1
    }
}
#[doc = "Field `ADERR` writer - Acknowledge Delimiter Error"]
pub type ADERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDC0ERFL_SPEC, ADERR_A, O>;
impl<'a, const O: u8> ADERR_W<'a, O> {
    #[doc = "Channel acknowledge delimiter error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADERR_A::_0)
    }
    #[doc = "Channel acknowledge delimiter error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADERR_A::_1)
    }
}
#[doc = "Field `CRCREG` reader - CRC Register value"]
pub type CRCREG_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Bus Error Flag"]
    #[inline(always)]
    pub fn bef(&self) -> BEF_R {
        BEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Warning Flag"]
    #[inline(always)]
    pub fn ewf(&self) -> EWF_R {
        EWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Passive Flag"]
    #[inline(always)]
    pub fn epf(&self) -> EPF_R {
        EPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus-Off Entry Flag"]
    #[inline(always)]
    pub fn boef(&self) -> BOEF_R {
        BOEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Flag"]
    #[inline(always)]
    pub fn borf(&self) -> BORF_R {
        BORF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overload Flag"]
    #[inline(always)]
    pub fn ovlf(&self) -> OVLF_R {
        OVLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus Lock Flag"]
    #[inline(always)]
    pub fn blf(&self) -> BLF_R {
        BLF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(&self) -> ALF_R {
        ALF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stuff Error"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Form Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge Error"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC Error"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit 1 Error"]
    #[inline(always)]
    pub fn b1err(&self) -> B1ERR_R {
        B1ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit 0 Error"]
    #[inline(always)]
    pub fn b0err(&self) -> B0ERR_R {
        B0ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Acknowledge Delimiter Error"]
    #[inline(always)]
    pub fn aderr(&self) -> ADERR_R {
        ADERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:30 - CRC Register value"]
    #[inline(always)]
    pub fn crcreg(&self) -> CRCREG_R {
        CRCREG_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bef(&mut self) -> BEF_W<0> {
        BEF_W::new(self)
    }
    #[doc = "Bit 1 - Error Warning Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ewf(&mut self) -> EWF_W<1> {
        EWF_W::new(self)
    }
    #[doc = "Bit 2 - Error Passive Flag"]
    #[inline(always)]
    #[must_use]
    pub fn epf(&mut self) -> EPF_W<2> {
        EPF_W::new(self)
    }
    #[doc = "Bit 3 - Bus-Off Entry Flag"]
    #[inline(always)]
    #[must_use]
    pub fn boef(&mut self) -> BOEF_W<3> {
        BOEF_W::new(self)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Flag"]
    #[inline(always)]
    #[must_use]
    pub fn borf(&mut self) -> BORF_W<4> {
        BORF_W::new(self)
    }
    #[doc = "Bit 5 - Overload Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovlf(&mut self) -> OVLF_W<5> {
        OVLF_W::new(self)
    }
    #[doc = "Bit 6 - Bus Lock Flag"]
    #[inline(always)]
    #[must_use]
    pub fn blf(&mut self) -> BLF_W<6> {
        BLF_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Lost Flag"]
    #[inline(always)]
    #[must_use]
    pub fn alf(&mut self) -> ALF_W<7> {
        ALF_W::new(self)
    }
    #[doc = "Bit 8 - Stuff Error"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SERR_W<8> {
        SERR_W::new(self)
    }
    #[doc = "Bit 9 - Form Error"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<9> {
        FERR_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge Error"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AERR_W<10> {
        AERR_W::new(self)
    }
    #[doc = "Bit 11 - CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cerr(&mut self) -> CERR_W<11> {
        CERR_W::new(self)
    }
    #[doc = "Bit 12 - Bit 1 Error"]
    #[inline(always)]
    #[must_use]
    pub fn b1err(&mut self) -> B1ERR_W<12> {
        B1ERR_W::new(self)
    }
    #[doc = "Bit 13 - Bit 0 Error"]
    #[inline(always)]
    #[must_use]
    pub fn b0err(&mut self) -> B0ERR_W<13> {
        B0ERR_W::new(self)
    }
    #[doc = "Bit 14 - Acknowledge Delimiter Error"]
    #[inline(always)]
    #[must_use]
    pub fn aderr(&mut self) -> ADERR_W<14> {
        ADERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 Error Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdc0erfl](index.html) module"]
pub struct CFDC0ERFL_SPEC;
impl crate::RegisterSpec for CFDC0ERFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdc0erfl::R](R) reader structure"]
impl crate::Readable for CFDC0ERFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdc0erfl::W](W) writer structure"]
impl crate::Writable for CFDC0ERFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDC0ERFL to value 0"]
impl crate::Resettable for CFDC0ERFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
