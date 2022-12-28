#[doc = "Register `ICCR1` reader"]
pub struct R(crate::R<ICCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCR1` writer"]
pub struct W(crate::W<ICCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCR1_SPEC>;
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
impl From<crate::W<ICCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDAI` reader - SDA Line Monitor"]
pub type SDAI_R = crate::BitReader<SDAI_A>;
#[doc = "SDA Line Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDAI_A {
    #[doc = "0: SDAn line is low"]
    _0 = 0,
    #[doc = "1: SDAn line is high"]
    _1 = 1,
}
impl From<SDAI_A> for bool {
    #[inline(always)]
    fn from(variant: SDAI_A) -> Self {
        variant as u8 != 0
    }
}
impl SDAI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAI_A {
        match self.bits {
            false => SDAI_A::_0,
            true => SDAI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDAI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDAI_A::_1
    }
}
#[doc = "Field `SCLI` reader - SCL Line Monitor"]
pub type SCLI_R = crate::BitReader<SCLI_A>;
#[doc = "SCL Line Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLI_A {
    #[doc = "0: SCLn line is low"]
    _0 = 0,
    #[doc = "1: SCLn line is high"]
    _1 = 1,
}
impl From<SCLI_A> for bool {
    #[inline(always)]
    fn from(variant: SCLI_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLI_A {
        match self.bits {
            false => SCLI_A::_0,
            true => SCLI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCLI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCLI_A::_1
    }
}
#[doc = "Field `SDAO` reader - SDA Output Control/Monitor"]
pub type SDAO_R = crate::BitReader<SDAO_A>;
#[doc = "SDA Output Control/Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDAO_A {
    #[doc = "0: Read: IIC drives SDAn pin low Write: IIC drives SDAn pin low"]
    _0 = 0,
    #[doc = "1: Read: IIC releases SDAn pin Write: IIC releases SDAn pin"]
    _1 = 1,
}
impl From<SDAO_A> for bool {
    #[inline(always)]
    fn from(variant: SDAO_A) -> Self {
        variant as u8 != 0
    }
}
impl SDAO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAO_A {
        match self.bits {
            false => SDAO_A::_0,
            true => SDAO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDAO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDAO_A::_1
    }
}
#[doc = "Field `SDAO` writer - SDA Output Control/Monitor"]
pub type SDAO_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR1_SPEC, SDAO_A, O>;
impl<'a, const O: u8> SDAO_W<'a, O> {
    #[doc = "Read: IIC drives SDAn pin low Write: IIC drives SDAn pin low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDAO_A::_0)
    }
    #[doc = "Read: IIC releases SDAn pin Write: IIC releases SDAn pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDAO_A::_1)
    }
}
#[doc = "Field `SCLO` reader - SCL Output Control/Monitor"]
pub type SCLO_R = crate::BitReader<SCLO_A>;
#[doc = "SCL Output Control/Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLO_A {
    #[doc = "0: Read: IIC drives SCLn pin low Write: IIC drives SCLn pin low"]
    _0 = 0,
    #[doc = "1: Read: IIC releases SCLn pin Write: IIC releases SCLn pin"]
    _1 = 1,
}
impl From<SCLO_A> for bool {
    #[inline(always)]
    fn from(variant: SCLO_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLO_A {
        match self.bits {
            false => SCLO_A::_0,
            true => SCLO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCLO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCLO_A::_1
    }
}
#[doc = "Field `SCLO` writer - SCL Output Control/Monitor"]
pub type SCLO_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR1_SPEC, SCLO_A, O>;
impl<'a, const O: u8> SCLO_W<'a, O> {
    #[doc = "Read: IIC drives SCLn pin low Write: IIC drives SCLn pin low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCLO_A::_0)
    }
    #[doc = "Read: IIC releases SCLn pin Write: IIC releases SCLn pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCLO_A::_1)
    }
}
#[doc = "SCLO/SDAO Write Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOWP_AW {
    #[doc = "0: Write enable SCLO and SDAO bits"]
    _0 = 0,
    #[doc = "1: Write protect SCLO and SDAO bits"]
    _1 = 1,
}
impl From<SOWP_AW> for bool {
    #[inline(always)]
    fn from(variant: SOWP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOWP` writer - SCLO/SDAO Write Protect"]
pub type SOWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR1_SPEC, SOWP_AW, O>;
impl<'a, const O: u8> SOWP_W<'a, O> {
    #[doc = "Write enable SCLO and SDAO bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOWP_AW::_0)
    }
    #[doc = "Write protect SCLO and SDAO bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOWP_AW::_1)
    }
}
#[doc = "Field `CLO` reader - Extra SCL Clock Cycle Output"]
pub type CLO_R = crate::BitReader<CLO_A>;
#[doc = "Extra SCL Clock Cycle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLO_A {
    #[doc = "0: Do not output extra SCL clock cycle (default)"]
    _0 = 0,
    #[doc = "1: Output extra SCL clock cycle"]
    _1 = 1,
}
impl From<CLO_A> for bool {
    #[inline(always)]
    fn from(variant: CLO_A) -> Self {
        variant as u8 != 0
    }
}
impl CLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLO_A {
        match self.bits {
            false => CLO_A::_0,
            true => CLO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLO_A::_1
    }
}
#[doc = "Field `CLO` writer - Extra SCL Clock Cycle Output"]
pub type CLO_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR1_SPEC, CLO_A, O>;
impl<'a, const O: u8> CLO_W<'a, O> {
    #[doc = "Do not output extra SCL clock cycle (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLO_A::_0)
    }
    #[doc = "Output extra SCL clock cycle"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLO_A::_1)
    }
}
#[doc = "Field `IICRST` reader - I2C Bus Interface Internal Reset"]
pub type IICRST_R = crate::BitReader<IICRST_A>;
#[doc = "I2C Bus Interface Internal Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICRST_A {
    #[doc = "0: Release IIC reset or internal reset"]
    _0 = 0,
    #[doc = "1: Initiate IIC reset or internal reset"]
    _1 = 1,
}
impl From<IICRST_A> for bool {
    #[inline(always)]
    fn from(variant: IICRST_A) -> Self {
        variant as u8 != 0
    }
}
impl IICRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICRST_A {
        match self.bits {
            false => IICRST_A::_0,
            true => IICRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICRST_A::_1
    }
}
#[doc = "Field `IICRST` writer - I2C Bus Interface Internal Reset"]
pub type IICRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR1_SPEC, IICRST_A, O>;
impl<'a, const O: u8> IICRST_W<'a, O> {
    #[doc = "Release IIC reset or internal reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICRST_A::_0)
    }
    #[doc = "Initiate IIC reset or internal reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICRST_A::_1)
    }
}
#[doc = "Field `ICE` reader - I2C Bus Interface Enable"]
pub type ICE_R = crate::BitReader<ICE_A>;
#[doc = "I2C Bus Interface Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICE_A {
    #[doc = "0: Disable (SCLn and SDAn pins in inactive state)"]
    _0 = 0,
    #[doc = "1: Enable (SCLn and SDAn pins in active state)"]
    _1 = 1,
}
impl From<ICE_A> for bool {
    #[inline(always)]
    fn from(variant: ICE_A) -> Self {
        variant as u8 != 0
    }
}
impl ICE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICE_A {
        match self.bits {
            false => ICE_A::_0,
            true => ICE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICE_A::_1
    }
}
#[doc = "Field `ICE` writer - I2C Bus Interface Enable"]
pub type ICE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICCR1_SPEC, ICE_A, O>;
impl<'a, const O: u8> ICE_W<'a, O> {
    #[doc = "Disable (SCLn and SDAn pins in inactive state)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICE_A::_0)
    }
    #[doc = "Enable (SCLn and SDAn pins in active state)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SDA Line Monitor"]
    #[inline(always)]
    pub fn sdai(&self) -> SDAI_R {
        SDAI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL Line Monitor"]
    #[inline(always)]
    pub fn scli(&self) -> SCLI_R {
        SCLI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDA Output Control/Monitor"]
    #[inline(always)]
    pub fn sdao(&self) -> SDAO_R {
        SDAO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCL Output Control/Monitor"]
    #[inline(always)]
    pub fn sclo(&self) -> SCLO_R {
        SCLO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn clo(&self) -> CLO_R {
        CLO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Bus Interface Internal Reset"]
    #[inline(always)]
    pub fn iicrst(&self) -> IICRST_R {
        IICRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Bus Interface Enable"]
    #[inline(always)]
    pub fn ice(&self) -> ICE_R {
        ICE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SDA Output Control/Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn sdao(&mut self) -> SDAO_W<2> {
        SDAO_W::new(self)
    }
    #[doc = "Bit 3 - SCL Output Control/Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn sclo(&mut self) -> SCLO_W<3> {
        SCLO_W::new(self)
    }
    #[doc = "Bit 4 - SCLO/SDAO Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sowp(&mut self) -> SOWP_W<4> {
        SOWP_W::new(self)
    }
    #[doc = "Bit 5 - Extra SCL Clock Cycle Output"]
    #[inline(always)]
    #[must_use]
    pub fn clo(&mut self) -> CLO_W<5> {
        CLO_W::new(self)
    }
    #[doc = "Bit 6 - I2C Bus Interface Internal Reset"]
    #[inline(always)]
    #[must_use]
    pub fn iicrst(&mut self) -> IICRST_W<6> {
        IICRST_W::new(self)
    }
    #[doc = "Bit 7 - I2C Bus Interface Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ice(&mut self) -> ICE_W<7> {
        ICE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccr1](index.html) module"]
pub struct ICCR1_SPEC;
impl crate::RegisterSpec for ICCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iccr1::R](R) reader structure"]
impl crate::Readable for ICCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccr1::W](W) writer structure"]
impl crate::Writable for ICCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICCR1 to value 0x1f"]
impl crate::Resettable for ICCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
