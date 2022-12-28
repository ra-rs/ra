#[doc = "Register `ADSHCR` reader"]
pub struct R(crate::R<ADSHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSHCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSHCR` writer"]
pub struct W(crate::W<ADSHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSHCR_SPEC>;
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
impl From<crate::W<ADSHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSHCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTSH` reader - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)"]
pub type SSTSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSTSH` writer - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)"]
pub type SSTSH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADSHCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SHANS0` reader - AN000 sample-and-hold circuit Select"]
pub type SHANS0_R = crate::BitReader<SHANS0_A>;
#[doc = "AN000 sample-and-hold circuit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHANS0_A {
    #[doc = "0: Bypass the sample-and-hold circuit."]
    _0 = 0,
    #[doc = "1: Use the sample-and-hold circuit."]
    _1 = 1,
}
impl From<SHANS0_A> for bool {
    #[inline(always)]
    fn from(variant: SHANS0_A) -> Self {
        variant as u8 != 0
    }
}
impl SHANS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHANS0_A {
        match self.bits {
            false => SHANS0_A::_0,
            true => SHANS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHANS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHANS0_A::_1
    }
}
#[doc = "Field `SHANS0` writer - AN000 sample-and-hold circuit Select"]
pub type SHANS0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADSHCR_SPEC, SHANS0_A, O>;
impl<'a, const O: u8> SHANS0_W<'a, O> {
    #[doc = "Bypass the sample-and-hold circuit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHANS0_A::_0)
    }
    #[doc = "Use the sample-and-hold circuit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHANS0_A::_1)
    }
}
#[doc = "Field `SHANS1` reader - AN001 sample-and-hold circuit Select"]
pub type SHANS1_R = crate::BitReader<SHANS1_A>;
#[doc = "AN001 sample-and-hold circuit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHANS1_A {
    #[doc = "0: Bypass the sample-and-hold circuit."]
    _0 = 0,
    #[doc = "1: Use the sample-and-hold circuit."]
    _1 = 1,
}
impl From<SHANS1_A> for bool {
    #[inline(always)]
    fn from(variant: SHANS1_A) -> Self {
        variant as u8 != 0
    }
}
impl SHANS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHANS1_A {
        match self.bits {
            false => SHANS1_A::_0,
            true => SHANS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHANS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHANS1_A::_1
    }
}
#[doc = "Field `SHANS1` writer - AN001 sample-and-hold circuit Select"]
pub type SHANS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADSHCR_SPEC, SHANS1_A, O>;
impl<'a, const O: u8> SHANS1_W<'a, O> {
    #[doc = "Bypass the sample-and-hold circuit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHANS1_A::_0)
    }
    #[doc = "Use the sample-and-hold circuit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHANS1_A::_1)
    }
}
#[doc = "Field `SHANS2` reader - AN002 sample-and-hold circuit Select"]
pub type SHANS2_R = crate::BitReader<SHANS2_A>;
#[doc = "AN002 sample-and-hold circuit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHANS2_A {
    #[doc = "0: Bypass the sample-and-hold circuit."]
    _0 = 0,
    #[doc = "1: Use the sample-and-hold circuit."]
    _1 = 1,
}
impl From<SHANS2_A> for bool {
    #[inline(always)]
    fn from(variant: SHANS2_A) -> Self {
        variant as u8 != 0
    }
}
impl SHANS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHANS2_A {
        match self.bits {
            false => SHANS2_A::_0,
            true => SHANS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHANS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHANS2_A::_1
    }
}
#[doc = "Field `SHANS2` writer - AN002 sample-and-hold circuit Select"]
pub type SHANS2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADSHCR_SPEC, SHANS2_A, O>;
impl<'a, const O: u8> SHANS2_W<'a, O> {
    #[doc = "Bypass the sample-and-hold circuit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHANS2_A::_0)
    }
    #[doc = "Use the sample-and-hold circuit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHANS2_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)"]
    #[inline(always)]
    pub fn sstsh(&self) -> SSTSH_R {
        SSTSH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - AN000 sample-and-hold circuit Select"]
    #[inline(always)]
    pub fn shans0(&self) -> SHANS0_R {
        SHANS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN001 sample-and-hold circuit Select"]
    #[inline(always)]
    pub fn shans1(&self) -> SHANS1_R {
        SHANS1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AN002 sample-and-hold circuit Select"]
    #[inline(always)]
    pub fn shans2(&self) -> SHANS2_R {
        SHANS2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)"]
    #[inline(always)]
    #[must_use]
    pub fn sstsh(&mut self) -> SSTSH_W<0> {
        SSTSH_W::new(self)
    }
    #[doc = "Bit 8 - AN000 sample-and-hold circuit Select"]
    #[inline(always)]
    #[must_use]
    pub fn shans0(&mut self) -> SHANS0_W<8> {
        SHANS0_W::new(self)
    }
    #[doc = "Bit 9 - AN001 sample-and-hold circuit Select"]
    #[inline(always)]
    #[must_use]
    pub fn shans1(&mut self) -> SHANS1_W<9> {
        SHANS1_W::new(self)
    }
    #[doc = "Bit 10 - AN002 sample-and-hold circuit Select"]
    #[inline(always)]
    #[must_use]
    pub fn shans2(&mut self) -> SHANS2_W<10> {
        SHANS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Sample and Hold Circuit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adshcr](index.html) module"]
pub struct ADSHCR_SPEC;
impl crate::RegisterSpec for ADSHCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adshcr::R](R) reader structure"]
impl crate::Readable for ADSHCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adshcr::W](W) writer structure"]
impl crate::Writable for ADSHCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSHCR to value 0x18"]
impl crate::Resettable for ADSHCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
