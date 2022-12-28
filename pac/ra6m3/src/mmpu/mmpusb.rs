#[doc = "Register `MMPUSB%s` reader"]
pub struct R(crate::R<MMPUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUSB%s` writer"]
pub struct W(crate::W<MMPUSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUSB_SPEC>;
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
impl From<crate::W<MMPUSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMPUSB` reader - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MMPUSB` writer - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMPUSB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusb(&self) -> MMPUSB_R {
        MMPUSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mmpusb(&mut self) -> MMPUSB_W<0> {
        MMPUSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Group B Region %s Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpusb](index.html) module"]
pub struct MMPUSB_SPEC;
impl crate::RegisterSpec for MMPUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmpusb::R](R) reader structure"]
impl crate::Readable for MMPUSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpusb::W](W) writer structure"]
impl crate::Writable for MMPUSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSB%s to value 0"]
impl crate::Resettable for MMPUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
