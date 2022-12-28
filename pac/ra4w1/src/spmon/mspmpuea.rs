#[doc = "Register `MSPMPUEA` reader"]
pub struct R(crate::R<MSPMPUEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSPMPUEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSPMPUEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSPMPUEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSPMPUEA` writer"]
pub struct W(crate::W<MSPMPUEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSPMPUEA_SPEC>;
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
impl From<crate::W<MSPMPUEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSPMPUEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSPMPUEA` reader - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type MSPMPUEA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MSPMPUEA` writer - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type MSPMPUEA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSPMPUEA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mspmpuea(&self) -> MSPMPUEA_R {
        MSPMPUEA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mspmpuea(&mut self) -> MSPMPUEA_W<0> {
        MSPMPUEA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Stack Pointer (MSP) Monitor End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspmpuea](index.html) module"]
pub struct MSPMPUEA_SPEC;
impl crate::RegisterSpec for MSPMPUEA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mspmpuea::R](R) reader structure"]
impl crate::Readable for MSPMPUEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mspmpuea::W](W) writer structure"]
impl crate::Writable for MSPMPUEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPMPUEA to value 0x03"]
impl crate::Resettable for MSPMPUEA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
