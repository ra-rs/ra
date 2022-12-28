#[doc = "Register `MSTPCRB` reader"]
pub struct R(crate::R<MSTPCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRB` writer"]
pub struct W(crate::W<MSTPCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRB_SPEC>;
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
impl From<crate::W<MSTPCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPB9` reader - IIC/I3C Bus Interface 0 Module Stop"]
pub type MSTPB9_R = crate::BitReader<MSTPB9_A>;
#[doc = "IIC/I3C Bus Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB9_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB9_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB9_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB9_A {
        match self.bits {
            false => MSTPB9_A::_0,
            true => MSTPB9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB9_A::_1
    }
}
#[doc = "Field `MSTPB9` writer - IIC/I3C Bus Interface 0 Module Stop"]
pub type MSTPB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB9_A, O>;
impl<'a, const O: u8> MSTPB9_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB9_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB9_A::_1)
    }
}
#[doc = "Field `MSTPB19` reader - Serial Peripheral Interface 0 Module Stop"]
pub type MSTPB19_R = crate::BitReader<MSTPB19_A>;
#[doc = "Serial Peripheral Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB19_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB19_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB19_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB19_A {
        match self.bits {
            false => MSTPB19_A::_0,
            true => MSTPB19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB19_A::_1
    }
}
#[doc = "Field `MSTPB19` writer - Serial Peripheral Interface 0 Module Stop"]
pub type MSTPB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB19_A, O>;
impl<'a, const O: u8> MSTPB19_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB19_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB19_A::_1)
    }
}
#[doc = "Field `MSTPB22` reader - Serial Communication Interface 9 Module Stop"]
pub type MSTPB22_R = crate::BitReader<MSTPB22_A>;
#[doc = "Serial Communication Interface 9 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB22_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPB22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB22_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPB22_A {
        match self.bits {
            false => MSTPB22_A::_0,
            true => MSTPB22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB22_A::_1
    }
}
#[doc = "Field `MSTPB22` writer - Serial Communication Interface 9 Module Stop"]
pub type MSTPB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRB_SPEC, MSTPB22_A, O>;
impl<'a, const O: u8> MSTPB22_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPB22_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPB22_A::_1)
    }
}
impl R {
    #[doc = "Bit 9 - IIC/I3C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb9(&self) -> MSTPB9_R {
        MSTPB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 19 - Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb19(&self) -> MSTPB19_R {
        MSTPB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    pub fn mstpb22(&self) -> MSTPB22_R {
        MSTPB22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - IIC/I3C Bus Interface 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb9(&mut self) -> MSTPB9_W<9> {
        MSTPB9_W::new(self)
    }
    #[doc = "Bit 19 - Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb19(&mut self) -> MSTPB19_W<19> {
        MSTPB19_W::new(self)
    }
    #[doc = "Bit 22 - Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpb22(&mut self) -> MSTPB22_W<22> {
        MSTPB22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcrb](index.html) module"]
pub struct MSTPCRB_SPEC;
impl crate::RegisterSpec for MSTPCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcrb::R](R) reader structure"]
impl crate::Readable for MSTPCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcrb::W](W) writer structure"]
impl crate::Writable for MSTPCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRB to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
