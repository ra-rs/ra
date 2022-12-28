#[doc = "Register `BUSWAIT` reader"]
pub struct R(crate::R<BUSWAIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSWAIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSWAIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSWAIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSWAIT` writer"]
pub struct W(crate::W<BUSWAIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSWAIT_SPEC>;
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
impl From<crate::W<BUSWAIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSWAIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BWAIT` reader - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)"]
pub type BWAIT_R = crate::FieldReader<u8, BWAIT_A>;
#[doc = "CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BWAIT_A(u8);
impl From<BWAIT_A> for u8 {
    #[inline(always)]
    fn from(val: BWAIT_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `BWAIT` writer - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)"]
pub type BWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BUSWAIT_SPEC, u8, BWAIT_A, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)"]
    #[inline(always)]
    pub fn bwait(&self) -> BWAIT_R {
        BWAIT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)"]
    #[inline(always)]
    #[must_use]
    pub fn bwait(&mut self) -> BWAIT_W<0> {
        BWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Bus Wait Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswait](index.html) module"]
pub struct BUSWAIT_SPEC;
impl crate::RegisterSpec for BUSWAIT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [buswait::R](R) reader structure"]
impl crate::Readable for BUSWAIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buswait::W](W) writer structure"]
impl crate::Writable for BUSWAIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSWAIT to value 0x0f"]
impl crate::Resettable for BUSWAIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
