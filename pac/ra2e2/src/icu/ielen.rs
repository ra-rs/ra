#[doc = "Register `IELEN` reader"]
pub struct R(crate::R<IELEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IELEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IELEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IELEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IELEN` writer"]
pub struct W(crate::W<IELEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IELEN_SPEC>;
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
impl From<crate::W<IELEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IELEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IELEN` reader - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
pub type IELEN_R = crate::BitReader<IELEN_A>;
#[doc = "Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IELEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<IELEN_A> for bool {
    #[inline(always)]
    fn from(variant: IELEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IELEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IELEN_A {
        match self.bits {
            false => IELEN_A::_0,
            true => IELEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IELEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IELEN_A::_1
    }
}
#[doc = "Field `IELEN` writer - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
pub type IELEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IELEN_SPEC, IELEN_A, O>;
impl<'a, const O: u8> IELEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IELEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IELEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
    #[inline(always)]
    pub fn ielen(&self) -> IELEN_R {
        IELEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn ielen(&mut self) -> IELEN_W<1> {
        IELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICU event Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ielen](index.html) module"]
pub struct IELEN_SPEC;
impl crate::RegisterSpec for IELEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ielen::R](R) reader structure"]
impl crate::Readable for IELEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ielen::W](W) writer structure"]
impl crate::Writable for IELEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IELEN to value 0"]
impl crate::Resettable for IELEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
