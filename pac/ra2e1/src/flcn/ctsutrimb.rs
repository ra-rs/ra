#[doc = "Register `CTSUTRIMB` reader"]
pub struct R(crate::R<CTSUTRIMB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUTRIMB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUTRIMB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUTRIMB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUTRIMB` writer"]
pub struct W(crate::W<CTSUTRIMB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUTRIMB_SPEC>;
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
impl From<crate::W<CTSUTRIMB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUTRIMB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRESULT0` reader - The coefficient of variation for the 7.5 kΩ reference load resistance is stored."]
pub type TRESULT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRESULT0` writer - The coefficient of variation for the 7.5 kΩ reference load resistance is stored."]
pub type TRESULT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMB_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRESULT1` reader - The coefficient of variation for the 15 kΩ reference load resistance is stored."]
pub type TRESULT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRESULT1` writer - The coefficient of variation for the 15 kΩ reference load resistance is stored."]
pub type TRESULT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMB_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRESULT2` reader - The coefficient of variation for the 30 kΩ reference load resistance is stored."]
pub type TRESULT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRESULT2` writer - The coefficient of variation for the 30 kΩ reference load resistance is stored."]
pub type TRESULT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMB_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRESULT3` reader - The coefficient of variation for the 60 kΩ reference load resistance is stored."]
pub type TRESULT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRESULT3` writer - The coefficient of variation for the 60 kΩ reference load resistance is stored."]
pub type TRESULT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUTRIMB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The coefficient of variation for the 7.5 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult0(&self) -> TRESULT0_R {
        TRESULT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The coefficient of variation for the 15 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult1(&self) -> TRESULT1_R {
        TRESULT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The coefficient of variation for the 30 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult2(&self) -> TRESULT2_R {
        TRESULT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The coefficient of variation for the 60 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult3(&self) -> TRESULT3_R {
        TRESULT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The coefficient of variation for the 7.5 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult0(&mut self) -> TRESULT0_W<0> {
        TRESULT0_W::new(self)
    }
    #[doc = "Bits 8:15 - The coefficient of variation for the 15 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult1(&mut self) -> TRESULT1_W<8> {
        TRESULT1_W::new(self)
    }
    #[doc = "Bits 16:23 - The coefficient of variation for the 30 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult2(&mut self) -> TRESULT2_W<16> {
        TRESULT2_W::new(self)
    }
    #[doc = "Bits 24:31 - The coefficient of variation for the 60 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult3(&mut self) -> TRESULT3_W<24> {
        TRESULT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Trimming Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsutrimb](index.html) module"]
pub struct CTSUTRIMB_SPEC;
impl crate::RegisterSpec for CTSUTRIMB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsutrimb::R](R) reader structure"]
impl crate::Readable for CTSUTRIMB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsutrimb::W](W) writer structure"]
impl crate::Writable for CTSUTRIMB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUTRIMB to value 0"]
impl crate::Resettable for CTSUTRIMB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
