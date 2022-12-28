#[doc = "Register `DMCRB` reader"]
pub struct R(crate::R<DMCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMCRB` writer"]
pub struct W(crate::W<DMCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMCRB_SPEC>;
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
impl From<crate::W<DMCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMCRBL` reader - Functions as a number of block, repeat or repeat-block transfer counter."]
pub type DMCRBL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMCRBL` writer - Functions as a number of block, repeat or repeat-block transfer counter."]
pub type DMCRBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCRB_SPEC, u16, u16, 16, O>;
#[doc = "Field `DMCRBH` reader - Specifies the number of block, repeat or repeat-block transfer operations."]
pub type DMCRBH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMCRBH` writer - Specifies the number of block, repeat or repeat-block transfer operations."]
pub type DMCRBH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCRB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Functions as a number of block, repeat or repeat-block transfer counter."]
    #[inline(always)]
    pub fn dmcrbl(&self) -> DMCRBL_R {
        DMCRBL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies the number of block, repeat or repeat-block transfer operations."]
    #[inline(always)]
    pub fn dmcrbh(&self) -> DMCRBH_R {
        DMCRBH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Functions as a number of block, repeat or repeat-block transfer counter."]
    #[inline(always)]
    #[must_use]
    pub fn dmcrbl(&mut self) -> DMCRBL_W<0> {
        DMCRBL_W::new(self)
    }
    #[doc = "Bits 16:31 - Specifies the number of block, repeat or repeat-block transfer operations."]
    #[inline(always)]
    #[must_use]
    pub fn dmcrbh(&mut self) -> DMCRBH_W<16> {
        DMCRBH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Block Transfer Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmcrb](index.html) module"]
pub struct DMCRB_SPEC;
impl crate::RegisterSpec for DMCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmcrb::R](R) reader structure"]
impl crate::Readable for DMCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmcrb::W](W) writer structure"]
impl crate::Writable for DMCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCRB to value 0"]
impl crate::Resettable for DMCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
