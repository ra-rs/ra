#[doc = "Register `SCKDIVCR2` reader"]
pub struct R(crate::R<SCKDIVCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCKDIVCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCKDIVCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCKDIVCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCKDIVCR2` writer"]
pub struct W(crate::W<SCKDIVCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCKDIVCR2_SPEC>;
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
impl From<crate::W<SCKDIVCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCKDIVCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCK` reader - USB Clock (UCLK) Select"]
pub type UCK_R = crate::FieldReader<u8, UCK_A>;
#[doc = "USB Clock (UCLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCK_A {
    #[doc = "2: /3"]
    _010 = 2,
    #[doc = "3: /4"]
    _011 = 3,
    #[doc = "4: /5"]
    _100 = 4,
}
impl From<UCK_A> for u8 {
    #[inline(always)]
    fn from(variant: UCK_A) -> Self {
        variant as _
    }
}
impl UCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UCK_A> {
        match self.bits {
            2 => Some(UCK_A::_010),
            3 => Some(UCK_A::_011),
            4 => Some(UCK_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == UCK_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == UCK_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == UCK_A::_100
    }
}
#[doc = "Field `UCK` writer - USB Clock (UCLK) Select"]
pub type UCK_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SCKDIVCR2_SPEC, u8, UCK_A, 3, O>;
impl<'a, const O: u8> UCK_W<'a, O> {
    #[doc = "/3"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(UCK_A::_010)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(UCK_A::_011)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(UCK_A::_100)
    }
}
impl R {
    #[doc = "Bits 4:6 - USB Clock (UCLK) Select"]
    #[inline(always)]
    pub fn uck(&self) -> UCK_R {
        UCK_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 4:6 - USB Clock (UCLK) Select"]
    #[inline(always)]
    #[must_use]
    pub fn uck(&mut self) -> UCK_W<4> {
        UCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Division Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sckdivcr2](index.html) module"]
pub struct SCKDIVCR2_SPEC;
impl crate::RegisterSpec for SCKDIVCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sckdivcr2::R](R) reader structure"]
impl crate::Readable for SCKDIVCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sckdivcr2::W](W) writer structure"]
impl crate::Writable for SCKDIVCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCKDIVCR2 to value 0x40"]
impl crate::Resettable for SCKDIVCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
