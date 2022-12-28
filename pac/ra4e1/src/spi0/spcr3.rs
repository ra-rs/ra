#[doc = "Register `SPCR3` reader"]
pub struct R(crate::R<SPCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR3` writer"]
pub struct W(crate::W<SPCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR3_SPEC>;
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
impl From<crate::W<SPCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETXMD` reader - Extended Communication Mode Select"]
pub type ETXMD_R = crate::BitReader<ETXMD_A>;
#[doc = "Extended Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETXMD_A {
    #[doc = "0: Full-duplex synchronous or transmit-only serial communications. \\[the SPCR.TXMD bit is enabled\\]"]
    _0 = 0,
    #[doc = "1: Receive-only serial communications in slave mode (SPCR.MSTR bit = 0). \\[the SPCR.TXMD bit is disabled\\]
Setting is prohibited in master mode (SPCR.MSTR bit = 1)."]
    _1 = 1,
}
impl From<ETXMD_A> for bool {
    #[inline(always)]
    fn from(variant: ETXMD_A) -> Self {
        variant as u8 != 0
    }
}
impl ETXMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETXMD_A {
        match self.bits {
            false => ETXMD_A::_0,
            true => ETXMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETXMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETXMD_A::_1
    }
}
#[doc = "Field `ETXMD` writer - Extended Communication Mode Select"]
pub type ETXMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR3_SPEC, ETXMD_A, O>;
impl<'a, const O: u8> ETXMD_W<'a, O> {
    #[doc = "Full-duplex synchronous or transmit-only serial communications. \\[the SPCR.TXMD bit is enabled\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETXMD_A::_0)
    }
    #[doc = "Receive-only serial communications in slave mode (SPCR.MSTR bit = 0). \\[the SPCR.TXMD bit is disabled\\]
Setting is prohibited in master mode (SPCR.MSTR bit = 1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETXMD_A::_1)
    }
}
#[doc = "Field `BFDS` reader - Between Burst Transfer Frames Delay Select"]
pub type BFDS_R = crate::BitReader<BFDS_A>;
#[doc = "Between Burst Transfer Frames Delay Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFDS_A {
    #[doc = "0: Delay (RSPCK delay, SSL negation delay and next-access delay) between frames is inserted in burst transfer."]
    _0 = 0,
    #[doc = "1: Delay between frames is not inserted in burst transfer."]
    _1 = 1,
}
impl From<BFDS_A> for bool {
    #[inline(always)]
    fn from(variant: BFDS_A) -> Self {
        variant as u8 != 0
    }
}
impl BFDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFDS_A {
        match self.bits {
            false => BFDS_A::_0,
            true => BFDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFDS_A::_1
    }
}
#[doc = "Field `BFDS` writer - Between Burst Transfer Frames Delay Select"]
pub type BFDS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR3_SPEC, BFDS_A, O>;
impl<'a, const O: u8> BFDS_W<'a, O> {
    #[doc = "Delay (RSPCK delay, SSL negation delay and next-access delay) between frames is inserted in burst transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFDS_A::_0)
    }
    #[doc = "Delay between frames is not inserted in burst transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFDS_A::_1)
    }
}
#[doc = "Field `CENDIE` reader - RSPI Communication End Interrupt Enable"]
pub type CENDIE_R = crate::BitReader<CENDIE_A>;
#[doc = "RSPI Communication End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENDIE_A {
    #[doc = "0: Communication end interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Communication end interrupt request is enabled."]
    _1 = 1,
}
impl From<CENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: CENDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CENDIE_A {
        match self.bits {
            false => CENDIE_A::_0,
            true => CENDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CENDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CENDIE_A::_1
    }
}
#[doc = "Field `CENDIE` writer - RSPI Communication End Interrupt Enable"]
pub type CENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR3_SPEC, CENDIE_A, O>;
impl<'a, const O: u8> CENDIE_W<'a, O> {
    #[doc = "Communication end interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CENDIE_A::_0)
    }
    #[doc = "Communication end interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CENDIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Extended Communication Mode Select"]
    #[inline(always)]
    pub fn etxmd(&self) -> ETXMD_R {
        ETXMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Between Burst Transfer Frames Delay Select"]
    #[inline(always)]
    pub fn bfds(&self) -> BFDS_R {
        BFDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RSPI Communication End Interrupt Enable"]
    #[inline(always)]
    pub fn cendie(&self) -> CENDIE_R {
        CENDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Extended Communication Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn etxmd(&mut self) -> ETXMD_W<0> {
        ETXMD_W::new(self)
    }
    #[doc = "Bit 1 - Between Burst Transfer Frames Delay Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfds(&mut self) -> BFDS_W<1> {
        BFDS_W::new(self)
    }
    #[doc = "Bit 4 - RSPI Communication End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cendie(&mut self) -> CENDIE_W<4> {
        CENDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr3](index.html) module"]
pub struct SPCR3_SPEC;
impl crate::RegisterSpec for SPCR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spcr3::R](R) reader structure"]
impl crate::Readable for SPCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr3::W](W) writer structure"]
impl crate::Writable for SPCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR3 to value 0"]
impl crate::Resettable for SPCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
