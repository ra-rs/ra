#[doc = "Register `FWEPROR` reader"]
pub struct R(crate::R<FWEPROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWEPROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWEPROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWEPROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWEPROR` writer"]
pub struct W(crate::W<FWEPROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWEPROR_SPEC>;
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
impl From<crate::W<FWEPROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWEPROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLWE` reader - Flash Programming and Erasure"]
pub type FLWE_R = crate::FieldReader<u8, FLWE_A>;
#[doc = "Flash Programming and Erasure\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLWE_A {
    #[doc = "0: Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    _00 = 0,
    #[doc = "1: Permits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    _01 = 1,
    #[doc = "2: Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    _10 = 2,
    #[doc = "3: Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    _11 = 3,
}
impl From<FLWE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLWE_A) -> Self {
        variant as _
    }
}
impl FLWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLWE_A {
        match self.bits {
            0 => FLWE_A::_00,
            1 => FLWE_A::_01,
            2 => FLWE_A::_10,
            3 => FLWE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLWE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLWE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLWE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLWE_A::_11
    }
}
#[doc = "Field `FLWE` writer - Flash Programming and Erasure"]
pub type FLWE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, FWEPROR_SPEC, u8, FLWE_A, 2, O>;
impl<'a, const O: u8> FLWE_W<'a, O> {
    #[doc = "Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLWE_A::_00)
    }
    #[doc = "Permits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLWE_A::_01)
    }
    #[doc = "Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLWE_A::_10)
    }
    #[doc = "Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLWE_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Programming and Erasure"]
    #[inline(always)]
    pub fn flwe(&self) -> FLWE_R {
        FLWE_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash Programming and Erasure"]
    #[inline(always)]
    #[must_use]
    pub fn flwe(&mut self) -> FLWE_W<0> {
        FLWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash P/E Protect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwepror](index.html) module"]
pub struct FWEPROR_SPEC;
impl crate::RegisterSpec for FWEPROR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fwepror::R](R) reader structure"]
impl crate::Readable for FWEPROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwepror::W](W) writer structure"]
impl crate::Writable for FWEPROR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWEPROR to value 0x02"]
impl crate::Resettable for FWEPROR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
