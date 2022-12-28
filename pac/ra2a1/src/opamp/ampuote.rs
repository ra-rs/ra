#[doc = "Register `AMPUOTE` reader"]
pub struct R(crate::R<AMPUOTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPUOTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPUOTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPUOTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPUOTE` writer"]
pub struct W(crate::W<AMPUOTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPUOTE_SPEC>;
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
impl From<crate::W<AMPUOTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPUOTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMP0TE` reader - AMP0OT write enable"]
pub type AMP0TE_R = crate::BitReader<AMP0TE_A>;
#[doc = "AMP0OT write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMP0TE_A {
    #[doc = "0: Not possible to write to the AMP0OTP and AMP0OTN registers"]
    _0 = 0,
    #[doc = "1: Possible to write to the AMP0OTP and AMP0OTN registers"]
    _1 = 1,
}
impl From<AMP0TE_A> for bool {
    #[inline(always)]
    fn from(variant: AMP0TE_A) -> Self {
        variant as u8 != 0
    }
}
impl AMP0TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMP0TE_A {
        match self.bits {
            false => AMP0TE_A::_0,
            true => AMP0TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMP0TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMP0TE_A::_1
    }
}
#[doc = "Field `AMP0TE` writer - AMP0OT write enable"]
pub type AMP0TE_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPUOTE_SPEC, AMP0TE_A, O>;
impl<'a, const O: u8> AMP0TE_W<'a, O> {
    #[doc = "Not possible to write to the AMP0OTP and AMP0OTN registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMP0TE_A::_0)
    }
    #[doc = "Possible to write to the AMP0OTP and AMP0OTN registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMP0TE_A::_1)
    }
}
#[doc = "Field `AMP1TE` reader - AMP1OT write enable"]
pub type AMP1TE_R = crate::BitReader<AMP1TE_A>;
#[doc = "AMP1OT write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMP1TE_A {
    #[doc = "0: Not possible to write to the AMP1OTP and AMP1OTN registers"]
    _0 = 0,
    #[doc = "1: Possible to write to the AMP1OTP and AMP1OTN registers"]
    _1 = 1,
}
impl From<AMP1TE_A> for bool {
    #[inline(always)]
    fn from(variant: AMP1TE_A) -> Self {
        variant as u8 != 0
    }
}
impl AMP1TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMP1TE_A {
        match self.bits {
            false => AMP1TE_A::_0,
            true => AMP1TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMP1TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMP1TE_A::_1
    }
}
#[doc = "Field `AMP1TE` writer - AMP1OT write enable"]
pub type AMP1TE_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPUOTE_SPEC, AMP1TE_A, O>;
impl<'a, const O: u8> AMP1TE_W<'a, O> {
    #[doc = "Not possible to write to the AMP1OTP and AMP1OTN registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMP1TE_A::_0)
    }
    #[doc = "Possible to write to the AMP1OTP and AMP1OTN registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMP1TE_A::_1)
    }
}
#[doc = "Field `AMP2TE` reader - AMP2OT write enable"]
pub type AMP2TE_R = crate::BitReader<AMP2TE_A>;
#[doc = "AMP2OT write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMP2TE_A {
    #[doc = "0: Not possible to write to the AMP2OTP and AMP2OTN registers"]
    _0 = 0,
    #[doc = "1: Possible to write to the AMP2OTP and AMP2OTN registers"]
    _1 = 1,
}
impl From<AMP2TE_A> for bool {
    #[inline(always)]
    fn from(variant: AMP2TE_A) -> Self {
        variant as u8 != 0
    }
}
impl AMP2TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMP2TE_A {
        match self.bits {
            false => AMP2TE_A::_0,
            true => AMP2TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMP2TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMP2TE_A::_1
    }
}
#[doc = "Field `AMP2TE` writer - AMP2OT write enable"]
pub type AMP2TE_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPUOTE_SPEC, AMP2TE_A, O>;
impl<'a, const O: u8> AMP2TE_W<'a, O> {
    #[doc = "Not possible to write to the AMP2OTP and AMP2OTN registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMP2TE_A::_0)
    }
    #[doc = "Possible to write to the AMP2OTP and AMP2OTN registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMP2TE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AMP0OT write enable"]
    #[inline(always)]
    pub fn amp0te(&self) -> AMP0TE_R {
        AMP0TE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AMP1OT write enable"]
    #[inline(always)]
    pub fn amp1te(&self) -> AMP1TE_R {
        AMP1TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AMP2OT write enable"]
    #[inline(always)]
    pub fn amp2te(&self) -> AMP2TE_R {
        AMP2TE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AMP0OT write enable"]
    #[inline(always)]
    #[must_use]
    pub fn amp0te(&mut self) -> AMP0TE_W<0> {
        AMP0TE_W::new(self)
    }
    #[doc = "Bit 1 - AMP1OT write enable"]
    #[inline(always)]
    #[must_use]
    pub fn amp1te(&mut self) -> AMP1TE_W<1> {
        AMP1TE_W::new(self)
    }
    #[doc = "Bit 2 - AMP2OT write enable"]
    #[inline(always)]
    #[must_use]
    pub fn amp2te(&mut self) -> AMP2TE_W<2> {
        AMP2TE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier User Offset Trimming Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampuote](index.html) module"]
pub struct AMPUOTE_SPEC;
impl crate::RegisterSpec for AMPUOTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ampuote::R](R) reader structure"]
impl crate::Readable for AMPUOTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampuote::W](W) writer structure"]
impl crate::Writable for AMPUOTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPUOTE to value 0"]
impl crate::Resettable for AMPUOTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
