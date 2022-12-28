#[doc = "Register `DAR` reader"]
pub struct R(crate::R<DAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAR` writer"]
pub struct W(crate::W<DAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAR_SPEC>;
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
impl From<crate::W<DAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVAD0` reader - Device Address data 0"]
pub type DVAD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVAD0` writer - Device Address data 0"]
pub type DVAD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DVAD1` reader - Device Address data 1"]
pub type DVAD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVAD1` writer - Device Address data 1"]
pub type DVAD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DVAD2` reader - Device Address data 2"]
pub type DVAD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVAD2` writer - Device Address data 2"]
pub type DVAD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DVAD3` reader - Device Address data 3"]
pub type DVAD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVAD3` writer - Device Address data 3"]
pub type DVAD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Device Address data 0"]
    #[inline(always)]
    pub fn dvad0(&self) -> DVAD0_R {
        DVAD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Device Address data 1"]
    #[inline(always)]
    pub fn dvad1(&self) -> DVAD1_R {
        DVAD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Device Address data 2"]
    #[inline(always)]
    pub fn dvad2(&self) -> DVAD2_R {
        DVAD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Device Address data 3"]
    #[inline(always)]
    pub fn dvad3(&self) -> DVAD3_R {
        DVAD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Device Address data 0"]
    #[inline(always)]
    #[must_use]
    pub fn dvad0(&mut self) -> DVAD0_W<0> {
        DVAD0_W::new(self)
    }
    #[doc = "Bits 8:15 - Device Address data 1"]
    #[inline(always)]
    #[must_use]
    pub fn dvad1(&mut self) -> DVAD1_W<8> {
        DVAD1_W::new(self)
    }
    #[doc = "Bits 16:23 - Device Address data 2"]
    #[inline(always)]
    #[must_use]
    pub fn dvad2(&mut self) -> DVAD2_W<16> {
        DVAD2_W::new(self)
    }
    #[doc = "Bits 24:31 - Device Address data 3"]
    #[inline(always)]
    #[must_use]
    pub fn dvad3(&mut self) -> DVAD3_W<24> {
        DVAD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dar](index.html) module"]
pub struct DAR_SPEC;
impl crate::RegisterSpec for DAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dar::R](R) reader structure"]
impl crate::Readable for DAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dar::W](W) writer structure"]
impl crate::Writable for DAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAR to value 0"]
impl crate::Resettable for DAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
