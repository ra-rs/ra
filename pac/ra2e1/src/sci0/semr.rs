#[doc = "Register `SEMR` reader"]
pub struct R(crate::R<SEMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEMR` writer"]
pub struct W(crate::W<SEMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEMR_SPEC>;
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
impl From<crate::W<SEMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRME` reader - Bit Rate Modulation Enable"]
pub type BRME_R = crate::BitReader<BRME_A>;
#[doc = "Bit Rate Modulation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRME_A {
    #[doc = "0: Disable bit rate modulation function"]
    _0 = 0,
    #[doc = "1: Enable bit rate modulation function"]
    _1 = 1,
}
impl From<BRME_A> for bool {
    #[inline(always)]
    fn from(variant: BRME_A) -> Self {
        variant as u8 != 0
    }
}
impl BRME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRME_A {
        match self.bits {
            false => BRME_A::_0,
            true => BRME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRME_A::_1
    }
}
#[doc = "Field `BRME` writer - Bit Rate Modulation Enable"]
pub type BRME_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEMR_SPEC, BRME_A, O>;
impl<'a, const O: u8> BRME_W<'a, O> {
    #[doc = "Disable bit rate modulation function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRME_A::_0)
    }
    #[doc = "Enable bit rate modulation function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRME_A::_1)
    }
}
#[doc = "Field `ABCSE` reader - Asynchronous Mode Extended Base Clock Select 1"]
pub type ABCSE_R = crate::BitReader<ABCSE_A>;
#[doc = "Asynchronous Mode Extended Base Clock Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABCSE_A {
    #[doc = "0: Clock cycles for 1-bit period determined by combination of the BGDM and ABCS bits in the SEMR register"]
    _0 = 0,
    #[doc = "1: Baud rate is 6 base clock cycles for 1-bit period"]
    _1 = 1,
}
impl From<ABCSE_A> for bool {
    #[inline(always)]
    fn from(variant: ABCSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ABCSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABCSE_A {
        match self.bits {
            false => ABCSE_A::_0,
            true => ABCSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCSE_A::_1
    }
}
#[doc = "Field `ABCSE` writer - Asynchronous Mode Extended Base Clock Select 1"]
pub type ABCSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEMR_SPEC, ABCSE_A, O>;
impl<'a, const O: u8> ABCSE_W<'a, O> {
    #[doc = "Clock cycles for 1-bit period determined by combination of the BGDM and ABCS bits in the SEMR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABCSE_A::_0)
    }
    #[doc = "Baud rate is 6 base clock cycles for 1-bit period"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABCSE_A::_1)
    }
}
#[doc = "Field `ABCS` reader - Asynchronous Mode Base Clock Select"]
pub type ABCS_R = crate::BitReader<ABCS_A>;
#[doc = "Asynchronous Mode Base Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABCS_A {
    #[doc = "0: Select 16 base clock cycles for 1-bit period"]
    _0 = 0,
    #[doc = "1: Select 8 base clock cycles for 1-bit period"]
    _1 = 1,
}
impl From<ABCS_A> for bool {
    #[inline(always)]
    fn from(variant: ABCS_A) -> Self {
        variant as u8 != 0
    }
}
impl ABCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABCS_A {
        match self.bits {
            false => ABCS_A::_0,
            true => ABCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCS_A::_1
    }
}
#[doc = "Field `ABCS` writer - Asynchronous Mode Base Clock Select"]
pub type ABCS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEMR_SPEC, ABCS_A, O>;
impl<'a, const O: u8> ABCS_W<'a, O> {
    #[doc = "Select 16 base clock cycles for 1-bit period"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABCS_A::_0)
    }
    #[doc = "Select 8 base clock cycles for 1-bit period"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABCS_A::_1)
    }
}
#[doc = "Field `NFEN` reader - Digital Noise Filter Function Enable"]
pub type NFEN_R = crate::BitReader<NFEN_A>;
#[doc = "Digital Noise Filter Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFEN_A {
    #[doc = "0: In asynchronous mode: Disable noise cancellation function for RXDn input signal In simple I2C mode: Disable noise cancellation function for SCLn and SDAn input signals"]
    _0 = 0,
    #[doc = "1: In asynchronous mode: Enable noise cancellation function for RXDn input signal In simple I2C mode: Enable noise cancellation function for SCLn and SDAn input signals"]
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
#[doc = "Field `NFEN` writer - Digital Noise Filter Function Enable"]
pub type NFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEMR_SPEC, NFEN_A, O>;
impl<'a, const O: u8> NFEN_W<'a, O> {
    #[doc = "In asynchronous mode: Disable noise cancellation function for RXDn input signal In simple I2C mode: Disable noise cancellation function for SCLn and SDAn input signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFEN_A::_0)
    }
    #[doc = "In asynchronous mode: Enable noise cancellation function for RXDn input signal In simple I2C mode: Enable noise cancellation function for SCLn and SDAn input signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFEN_A::_1)
    }
}
#[doc = "Field `BGDM` reader - Baud Rate Generator Double-Speed Mode Select"]
pub type BGDM_R = crate::BitReader<BGDM_A>;
#[doc = "Baud Rate Generator Double-Speed Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGDM_A {
    #[doc = "0: Output clock from baud rate generator with normal frequency"]
    _0 = 0,
    #[doc = "1: Output clock from baud rate generator with doubled frequency"]
    _1 = 1,
}
impl From<BGDM_A> for bool {
    #[inline(always)]
    fn from(variant: BGDM_A) -> Self {
        variant as u8 != 0
    }
}
impl BGDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGDM_A {
        match self.bits {
            false => BGDM_A::_0,
            true => BGDM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGDM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGDM_A::_1
    }
}
#[doc = "Field `BGDM` writer - Baud Rate Generator Double-Speed Mode Select"]
pub type BGDM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEMR_SPEC, BGDM_A, O>;
impl<'a, const O: u8> BGDM_W<'a, O> {
    #[doc = "Output clock from baud rate generator with normal frequency"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGDM_A::_0)
    }
    #[doc = "Output clock from baud rate generator with doubled frequency"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGDM_A::_1)
    }
}
#[doc = "Field `RXDESEL` reader - Asynchronous Start Bit Edge Detection Select"]
pub type RXDESEL_R = crate::BitReader<RXDESEL_A>;
#[doc = "Asynchronous Start Bit Edge Detection Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDESEL_A {
    #[doc = "0: Detect low level on RXDn pin as start bit"]
    _0 = 0,
    #[doc = "1: Detect falling edge of RXDn pin as start bit"]
    _1 = 1,
}
impl From<RXDESEL_A> for bool {
    #[inline(always)]
    fn from(variant: RXDESEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDESEL_A {
        match self.bits {
            false => RXDESEL_A::_0,
            true => RXDESEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDESEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDESEL_A::_1
    }
}
#[doc = "Field `RXDESEL` writer - Asynchronous Start Bit Edge Detection Select"]
pub type RXDESEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEMR_SPEC, RXDESEL_A, O>;
impl<'a, const O: u8> RXDESEL_W<'a, O> {
    #[doc = "Detect low level on RXDn pin as start bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDESEL_A::_0)
    }
    #[doc = "Detect falling edge of RXDn pin as start bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDESEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Bit Rate Modulation Enable"]
    #[inline(always)]
    pub fn brme(&self) -> BRME_R {
        BRME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Mode Extended Base Clock Select 1"]
    #[inline(always)]
    pub fn abcse(&self) -> ABCSE_R {
        ABCSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(&self) -> ABCS_R {
        ABCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(&self) -> BGDM_R {
        BGDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(&self) -> RXDESEL_R {
        RXDESEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Bit Rate Modulation Enable"]
    #[inline(always)]
    pub fn brme(&mut self) -> BRME_W<2> {
        BRME_W::new(self)
    }
    #[doc = "Bit 3 - Asynchronous Mode Extended Base Clock Select 1"]
    #[inline(always)]
    pub fn abcse(&mut self) -> ABCSE_W<3> {
        ABCSE_W::new(self)
    }
    #[doc = "Bit 4 - Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(&mut self) -> ABCS_W<4> {
        ABCS_W::new(self)
    }
    #[doc = "Bit 5 - Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(&mut self) -> NFEN_W<5> {
        NFEN_W::new(self)
    }
    #[doc = "Bit 6 - Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(&mut self) -> BGDM_W<6> {
        BGDM_W::new(self)
    }
    #[doc = "Bit 7 - Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(&mut self) -> RXDESEL_W<7> {
        RXDESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semr](index.html) module"]
pub struct SEMR_SPEC;
impl crate::RegisterSpec for SEMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [semr::R](R) reader structure"]
impl crate::Readable for SEMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [semr::W](W) writer structure"]
impl crate::Writable for SEMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEMR to value 0"]
impl crate::Resettable for SEMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
