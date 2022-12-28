#[doc = "Register `SRCODCTRL` reader"]
pub struct R(crate::R<SRCODCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCODCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCODCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCODCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCODCTRL` writer"]
pub struct W(crate::W<SRCODCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCODCTRL_SPEC>;
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
impl From<crate::W<SRCODCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCODCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFTRG` reader - Output FIFO Data Trigger Number"]
pub type OFTRG_R = crate::FieldReader<u8, OFTRG_A>;
#[doc = "Output FIFO Data Trigger Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OFTRG_A {
    #[doc = "0: 1"]
    _00 = 0,
    #[doc = "1: 4"]
    _01 = 1,
    #[doc = "2: 8"]
    _10 = 2,
    #[doc = "3: 12"]
    _11 = 3,
}
impl From<OFTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: OFTRG_A) -> Self {
        variant as _
    }
}
impl OFTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFTRG_A {
        match self.bits {
            0 => OFTRG_A::_00,
            1 => OFTRG_A::_01,
            2 => OFTRG_A::_10,
            3 => OFTRG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OFTRG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OFTRG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OFTRG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OFTRG_A::_11
    }
}
#[doc = "Field `OFTRG` writer - Output FIFO Data Trigger Number"]
pub type OFTRG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SRCODCTRL_SPEC, u8, OFTRG_A, 2, O>;
impl<'a, const O: u8> OFTRG_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OFTRG_A::_00)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OFTRG_A::_01)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OFTRG_A::_10)
    }
    #[doc = "12"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OFTRG_A::_11)
    }
}
#[doc = "Field `OEN` reader - Output Data FIFO Full Interrupt Enable"]
pub type OEN_R = crate::BitReader<OEN_A>;
#[doc = "Output Data FIFO Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OEN_A {
    #[doc = "0: Output data FIFO full interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Output data FIFO full interrupt is enabled."]
    _1 = 1,
}
impl From<OEN_A> for bool {
    #[inline(always)]
    fn from(variant: OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEN_A {
        match self.bits {
            false => OEN_A::_0,
            true => OEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OEN_A::_1
    }
}
#[doc = "Field `OEN` writer - Output Data FIFO Full Interrupt Enable"]
pub type OEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCODCTRL_SPEC, OEN_A, O>;
impl<'a, const O: u8> OEN_W<'a, O> {
    #[doc = "Output data FIFO full interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OEN_A::_0)
    }
    #[doc = "Output data FIFO full interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OEN_A::_1)
    }
}
#[doc = "Field `OED` reader - Output Data Endian"]
pub type OED_R = crate::BitReader<OED_A>;
#[doc = "Output Data Endian\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OED_A {
    #[doc = "0: Endian formats are the same between the chip and input data."]
    _0 = 0,
    #[doc = "1: Endian formats are different between the chip and input data."]
    _1 = 1,
}
impl From<OED_A> for bool {
    #[inline(always)]
    fn from(variant: OED_A) -> Self {
        variant as u8 != 0
    }
}
impl OED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OED_A {
        match self.bits {
            false => OED_A::_0,
            true => OED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OED_A::_1
    }
}
#[doc = "Field `OED` writer - Output Data Endian"]
pub type OED_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCODCTRL_SPEC, OED_A, O>;
impl<'a, const O: u8> OED_W<'a, O> {
    #[doc = "Endian formats are the same between the chip and input data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OED_A::_0)
    }
    #[doc = "Endian formats are different between the chip and input data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OED_A::_1)
    }
}
#[doc = "Field `OCH` reader - Output Data Channel Exchange"]
pub type OCH_R = crate::BitReader<OCH_A>;
#[doc = "Output Data Channel Exchange\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCH_A {
    #[doc = "0: Does not exchange the channels (the same order as data input)"]
    _0 = 0,
    #[doc = "1: Exchanges the channels (the opposite order from data input)"]
    _1 = 1,
}
impl From<OCH_A> for bool {
    #[inline(always)]
    fn from(variant: OCH_A) -> Self {
        variant as u8 != 0
    }
}
impl OCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCH_A {
        match self.bits {
            false => OCH_A::_0,
            true => OCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCH_A::_1
    }
}
#[doc = "Field `OCH` writer - Output Data Channel Exchange"]
pub type OCH_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCODCTRL_SPEC, OCH_A, O>;
impl<'a, const O: u8> OCH_W<'a, O> {
    #[doc = "Does not exchange the channels (the same order as data input)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCH_A::_0)
    }
    #[doc = "Exchanges the channels (the opposite order from data input)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCH_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Output FIFO Data Trigger Number"]
    #[inline(always)]
    pub fn oftrg(&self) -> OFTRG_R {
        OFTRG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Output Data FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Data Endian"]
    #[inline(always)]
    pub fn oed(&self) -> OED_R {
        OED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Data Channel Exchange"]
    #[inline(always)]
    pub fn och(&self) -> OCH_R {
        OCH_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output FIFO Data Trigger Number"]
    #[inline(always)]
    #[must_use]
    pub fn oftrg(&mut self) -> OFTRG_W<0> {
        OFTRG_W::new(self)
    }
    #[doc = "Bit 8 - Output Data FIFO Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OEN_W<8> {
        OEN_W::new(self)
    }
    #[doc = "Bit 9 - Output Data Endian"]
    #[inline(always)]
    #[must_use]
    pub fn oed(&mut self) -> OED_W<9> {
        OED_W::new(self)
    }
    #[doc = "Bit 10 - Output Data Channel Exchange"]
    #[inline(always)]
    #[must_use]
    pub fn och(&mut self) -> OCH_W<10> {
        OCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Data Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcodctrl](index.html) module"]
pub struct SRCODCTRL_SPEC;
impl crate::RegisterSpec for SRCODCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [srcodctrl::R](R) reader structure"]
impl crate::Readable for SRCODCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcodctrl::W](W) writer structure"]
impl crate::Writable for SRCODCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCODCTRL to value 0"]
impl crate::Resettable for SRCODCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
