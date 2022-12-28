#[doc = "Register `DCSMXR` reader"]
pub struct R(crate::R<DCSMXR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCSMXR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCSMXR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCSMXR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCSMXR` writer"]
pub struct W(crate::W<DCSMXR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCSMXR_SPEC>;
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
impl From<crate::W<DCSMXR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCSMXR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTWMX0` reader - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM."]
pub type CTWMX0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTWMX0` writer - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM."]
pub type CTWMX0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCSMXR_SPEC, u16, u16, 9, O>;
#[doc = "Field `CTWMX1` reader - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM."]
pub type CTWMX1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTWMX1` writer - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM."]
pub type CTWMX1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCSMXR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM."]
    #[inline(always)]
    pub fn ctwmx0(&self) -> CTWMX0_R {
        CTWMX0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM."]
    #[inline(always)]
    pub fn ctwmx1(&self) -> CTWMX1_R {
        CTWMX1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM."]
    #[inline(always)]
    #[must_use]
    pub fn ctwmx0(&mut self) -> CTWMX0_W<0> {
        CTWMX0_W::new(self)
    }
    #[doc = "Bits 16:24 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM."]
    #[inline(always)]
    #[must_use]
    pub fn ctwmx1(&mut self) -> CTWMX1_W<16> {
        CTWMX1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Chip Select Maximum Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcsmxr](index.html) module"]
pub struct DCSMXR_SPEC;
impl crate::RegisterSpec for DCSMXR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcsmxr::R](R) reader structure"]
impl crate::Readable for DCSMXR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcsmxr::W](W) writer structure"]
impl crate::Writable for DCSMXR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCSMXR to value 0"]
impl crate::Resettable for DCSMXR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
