#[doc = "Register `DEVADD%s` reader"]
pub struct R(crate::R<DEVADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVADD%s` writer"]
pub struct W(crate::W<DEVADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVADD_SPEC>;
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
impl From<crate::W<DEVADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBSPD` reader - Transfer Speed of Communication Target Device"]
pub type USBSPD_R = crate::FieldReader<u8, USBSPD_A>;
#[doc = "Transfer Speed of Communication Target Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSPD_A {
    #[doc = "0: Do not use DEVADDn"]
    _00 = 0,
    #[doc = "1: Low-speed"]
    _01 = 1,
    #[doc = "2: Full-speed"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<USBSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSPD_A) -> Self {
        variant as _
    }
}
impl USBSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSPD_A {
        match self.bits {
            0 => USBSPD_A::_00,
            1 => USBSPD_A::_01,
            2 => USBSPD_A::_10,
            3 => USBSPD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == USBSPD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == USBSPD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == USBSPD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == USBSPD_A::_11
    }
}
#[doc = "Field `USBSPD` writer - Transfer Speed of Communication Target Device"]
pub type USBSPD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DEVADD_SPEC, u8, USBSPD_A, 2, O>;
impl<'a, const O: u8> USBSPD_W<'a, O> {
    #[doc = "Do not use DEVADDn"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(USBSPD_A::_00)
    }
    #[doc = "Low-speed"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(USBSPD_A::_01)
    }
    #[doc = "Full-speed"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(USBSPD_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(USBSPD_A::_11)
    }
}
impl R {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub fn usbspd(&self) -> USBSPD_R {
        USBSPD_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    #[must_use]
    pub fn usbspd(&mut self) -> USBSPD_W<6> {
        USBSPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Address %s Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devadd](index.html) module"]
pub struct DEVADD_SPEC;
impl crate::RegisterSpec for DEVADD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [devadd::R](R) reader structure"]
impl crate::Readable for DEVADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devadd::W](W) writer structure"]
impl crate::Writable for DEVADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVADD%s to value 0"]
impl crate::Resettable for DEVADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
