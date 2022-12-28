#[doc = "Register `JINTS0` reader"]
pub struct R(crate::R<JINTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JINTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JINTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JINTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JINTS0` writer"]
pub struct W(crate::W<JINTS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JINTS0_SPEC>;
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
impl From<crate::W<JINTS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JINTS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INS3` reader - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD.\n\nThe field is **modified** in some way after a read operation."]
pub type INS3_R = crate::BitReader<bool>;
#[doc = "Field `INS3` writer - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD."]
pub type INS3_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, JINTS0_SPEC, bool, O>;
#[doc = "Field `INS5` reader - This bit is set to 1 when a compressed data error occurs.\n\nThe field is **modified** in some way after a read operation."]
pub type INS5_R = crate::BitReader<bool>;
#[doc = "Field `INS5` writer - This bit is set to 1 when a compressed data error occurs."]
pub type INS5_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, JINTS0_SPEC, bool, O>;
#[doc = "Field `INS6` reader - This bit is set to 1 when this module completes compression process normally.\n\nThe field is **modified** in some way after a read operation."]
pub type INS6_R = crate::BitReader<bool>;
#[doc = "Field `INS6` writer - This bit is set to 1 when this module completes compression process normally."]
pub type INS6_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, JINTS0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD."]
    #[inline(always)]
    pub fn ins3(&self) -> INS3_R {
        INS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to 1 when a compressed data error occurs."]
    #[inline(always)]
    pub fn ins5(&self) -> INS5_R {
        INS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set to 1 when this module completes compression process normally."]
    #[inline(always)]
    pub fn ins6(&self) -> INS6_R {
        INS6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD."]
    #[inline(always)]
    #[must_use]
    pub fn ins3(&mut self) -> INS3_W<3> {
        INS3_W::new(self)
    }
    #[doc = "Bit 5 - This bit is set to 1 when a compressed data error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ins5(&mut self) -> INS5_W<5> {
        INS5_W::new(self)
    }
    #[doc = "Bit 6 - This bit is set to 1 when this module completes compression process normally."]
    #[inline(always)]
    #[must_use]
    pub fn ins6(&mut self) -> INS6_W<6> {
        INS6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interrupt Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jints0](index.html) module"]
pub struct JINTS0_SPEC;
impl crate::RegisterSpec for JINTS0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jints0::R](R) reader structure"]
impl crate::Readable for JINTS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jints0::W](W) writer structure"]
impl crate::Writable for JINTS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x68;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JINTS0 to value 0"]
impl crate::Resettable for JINTS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
