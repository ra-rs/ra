#[doc = "Register `LCIVRL` reader"]
pub struct R(crate::R<LCIVRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCIVRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCIVRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCIVRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCIVRL` writer"]
pub struct W(crate::W<LCIVRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCIVRL_SPEC>;
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
impl From<crate::W<LCIVRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCIVRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCIVRL` reader - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds."]
pub type LCIVRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LCIVRL` writer - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds."]
pub type LCIVRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCIVRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds."]
    #[inline(always)]
    pub fn lcivrl(&self) -> LCIVRL_R {
        LCIVRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds."]
    #[inline(always)]
    #[must_use]
    pub fn lcivrl(&mut self) -> LCIVRL_W<0> {
        LCIVRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local Time Counter Initial Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcivrl](index.html) module"]
pub struct LCIVRL_SPEC;
impl crate::RegisterSpec for LCIVRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcivrl::R](R) reader structure"]
impl crate::Readable for LCIVRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcivrl::W](W) writer structure"]
impl crate::Writable for LCIVRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCIVRL to value 0"]
impl crate::Resettable for LCIVRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
