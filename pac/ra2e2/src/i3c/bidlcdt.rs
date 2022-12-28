#[doc = "Register `BIDLCDT` reader"]
pub struct R(crate::R<BIDLCDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIDLCDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIDLCDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIDLCDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIDLCDT` writer"]
pub struct W(crate::W<BIDLCDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIDLCDT_SPEC>;
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
impl From<crate::W<BIDLCDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIDLCDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLCYC` reader - Bus Idle Condition Detection Cycle"]
pub type IDLCYC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDLCYC` writer - Bus Idle Condition Detection Cycle"]
pub type IDLCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIDLCDT_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - Bus Idle Condition Detection Cycle"]
    #[inline(always)]
    pub fn idlcyc(&self) -> IDLCYC_R {
        IDLCYC_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Bus Idle Condition Detection Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn idlcyc(&mut self) -> IDLCYC_W<0> {
        IDLCYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Idle Condition Detection Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bidlcdt](index.html) module"]
pub struct BIDLCDT_SPEC;
impl crate::RegisterSpec for BIDLCDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bidlcdt::R](R) reader structure"]
impl crate::Readable for BIDLCDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bidlcdt::W](W) writer structure"]
impl crate::Writable for BIDLCDT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIDLCDT to value 0"]
impl crate::Resettable for BIDLCDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
