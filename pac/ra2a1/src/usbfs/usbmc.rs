#[doc = "Register `USBMC` reader"]
pub struct R(crate::R<USBMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBMC` writer"]
pub struct W(crate::W<USBMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBMC_SPEC>;
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
impl From<crate::W<USBMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDUSBE` reader - USB Reference Power Supply Circuit On/Off Control"]
pub type VDDUSBE_R = crate::BitReader<VDDUSBE_A>;
#[doc = "USB Reference Power Supply Circuit On/Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDUSBE_A {
    #[doc = "0: USB reference power supply circuit off"]
    _0 = 0,
    #[doc = "1: USB reference power supply circuit on"]
    _1 = 1,
}
impl From<VDDUSBE_A> for bool {
    #[inline(always)]
    fn from(variant: VDDUSBE_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDUSBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDUSBE_A {
        match self.bits {
            false => VDDUSBE_A::_0,
            true => VDDUSBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDDUSBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDDUSBE_A::_1
    }
}
#[doc = "Field `VDDUSBE` writer - USB Reference Power Supply Circuit On/Off Control"]
pub type VDDUSBE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBMC_SPEC, VDDUSBE_A, O>;
impl<'a, const O: u8> VDDUSBE_W<'a, O> {
    #[doc = "USB reference power supply circuit off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDDUSBE_A::_0)
    }
    #[doc = "USB reference power supply circuit on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDDUSBE_A::_1)
    }
}
#[doc = "Field `VDCEN` reader - USB Regulator On/Off Control"]
pub type VDCEN_R = crate::BitReader<VDCEN_A>;
#[doc = "USB Regulator On/Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDCEN_A {
    #[doc = "0: USB regulator off"]
    _0 = 0,
    #[doc = "1: USB regulator on"]
    _1 = 1,
}
impl From<VDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: VDCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDCEN_A {
        match self.bits {
            false => VDCEN_A::_0,
            true => VDCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDCEN_A::_1
    }
}
#[doc = "Field `VDCEN` writer - USB Regulator On/Off Control"]
pub type VDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBMC_SPEC, VDCEN_A, O>;
impl<'a, const O: u8> VDCEN_W<'a, O> {
    #[doc = "USB regulator off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDCEN_A::_0)
    }
    #[doc = "USB regulator on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDCEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    pub fn vddusbe(&self) -> VDDUSBE_R {
        VDDUSBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - USB Regulator On/Off Control"]
    #[inline(always)]
    pub fn vdcen(&self) -> VDCEN_R {
        VDCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    #[must_use]
    pub fn vddusbe(&mut self) -> VDDUSBE_W<0> {
        VDDUSBE_W::new(self)
    }
    #[doc = "Bit 7 - USB Regulator On/Off Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdcen(&mut self) -> VDCEN_W<7> {
        VDCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbmc](index.html) module"]
pub struct USBMC_SPEC;
impl crate::RegisterSpec for USBMC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbmc::R](R) reader structure"]
impl crate::Readable for USBMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbmc::W](W) writer structure"]
impl crate::Writable for USBMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBMC to value 0x02"]
impl crate::Resettable for USBMC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
