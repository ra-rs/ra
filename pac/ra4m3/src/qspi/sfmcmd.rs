#[doc = "Register `SFMCMD` reader"]
pub struct R(crate::R<SFMCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMCMD` writer"]
pub struct W(crate::W<SFMCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMCMD_SPEC>;
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
impl From<crate::W<SFMCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOM` reader - Mode select for communication with the SPI bus"]
pub type DCOM_R = crate::BitReader<DCOM_A>;
#[doc = "Mode select for communication with the SPI bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOM_A {
    #[doc = "0: ROM access mode"]
    _0 = 0,
    #[doc = "1: Direct communication mode"]
    _1 = 1,
}
impl From<DCOM_A> for bool {
    #[inline(always)]
    fn from(variant: DCOM_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOM_A {
        match self.bits {
            false => DCOM_A::_0,
            true => DCOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCOM_A::_1
    }
}
#[doc = "Field `DCOM` writer - Mode select for communication with the SPI bus"]
pub type DCOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMCMD_SPEC, DCOM_A, O>;
impl<'a, const O: u8> DCOM_W<'a, O> {
    #[doc = "ROM access mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOM_A::_0)
    }
    #[doc = "Direct communication mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mode select for communication with the SPI bus"]
    #[inline(always)]
    pub fn dcom(&self) -> DCOM_R {
        DCOM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode select for communication with the SPI bus"]
    #[inline(always)]
    #[must_use]
    pub fn dcom(&mut self) -> DCOM_W<0> {
        DCOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Communication Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmcmd](index.html) module"]
pub struct SFMCMD_SPEC;
impl crate::RegisterSpec for SFMCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmcmd::R](R) reader structure"]
impl crate::Readable for SFMCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmcmd::W](W) writer structure"]
impl crate::Writable for SFMCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMCMD to value 0"]
impl crate::Resettable for SFMCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
