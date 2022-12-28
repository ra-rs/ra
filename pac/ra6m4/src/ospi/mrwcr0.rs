#[doc = "Register `MRWCR0` reader"]
pub struct R(crate::R<MRWCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRWCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRWCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRWCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRWCR0` writer"]
pub struct W(crate::W<MRWCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRWCR0_SPEC>;
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
impl From<crate::W<MRWCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRWCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D0MRCMD0` reader - Memory map read command 0 setting"]
pub type D0MRCMD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D0MRCMD0` writer - Memory map read command 0 setting"]
pub type D0MRCMD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `D0MRCMD1` reader - Memory map read command 1 setting"]
pub type D0MRCMD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D0MRCMD1` writer - Memory map read command 1 setting"]
pub type D0MRCMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `D0MWCMD0` reader - Memory map write command 0 setting"]
pub type D0MWCMD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D0MWCMD0` writer - Memory map write command 0 setting"]
pub type D0MWCMD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `D0MWCMD1` reader - Memory map write command 1 setting"]
pub type D0MWCMD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D0MWCMD1` writer - Memory map write command 1 setting"]
pub type D0MWCMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Memory map read command 0 setting"]
    #[inline(always)]
    pub fn d0mrcmd0(&self) -> D0MRCMD0_R {
        D0MRCMD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Memory map read command 1 setting"]
    #[inline(always)]
    pub fn d0mrcmd1(&self) -> D0MRCMD1_R {
        D0MRCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Memory map write command 0 setting"]
    #[inline(always)]
    pub fn d0mwcmd0(&self) -> D0MWCMD0_R {
        D0MWCMD0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Memory map write command 1 setting"]
    #[inline(always)]
    pub fn d0mwcmd1(&self) -> D0MWCMD1_R {
        D0MWCMD1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Memory map read command 0 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d0mrcmd0(&mut self) -> D0MRCMD0_W<0> {
        D0MRCMD0_W::new(self)
    }
    #[doc = "Bits 8:15 - Memory map read command 1 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d0mrcmd1(&mut self) -> D0MRCMD1_W<8> {
        D0MRCMD1_W::new(self)
    }
    #[doc = "Bits 16:23 - Memory map write command 0 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d0mwcmd0(&mut self) -> D0MWCMD0_W<16> {
        D0MWCMD0_W::new(self)
    }
    #[doc = "Bits 24:31 - Memory map write command 1 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d0mwcmd1(&mut self) -> D0MWCMD1_W<24> {
        D0MWCMD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Map Read/Write Command Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrwcr0](index.html) module"]
pub struct MRWCR0_SPEC;
impl crate::RegisterSpec for MRWCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrwcr0::R](R) reader structure"]
impl crate::Readable for MRWCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrwcr0::W](W) writer structure"]
impl crate::Writable for MRWCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRWCR0 to value 0"]
impl crate::Resettable for MRWCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
